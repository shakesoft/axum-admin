use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::utils::jwt_util::JwtToken;
use crate::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{http, response, Json};
use redis::{Client};
use std::sync::Arc;
use log::info;
use crate::utils::jwt_util;
use crate::service::system::sys_user_service::SysUserService;

#[function_name::named]
pub async fn auth(State(state): State<Arc<AppState>>, mut req: Request, next: Next) -> Result<response::Response, StatusCode> {
    let path = req.uri().path();
    info!("{function_name}:request_uri {uri:?}",function_name = function_name!(),uri = path);
    let ignore_paths = vec![
        "/system/user/login",
        "/system/user/register",
        "/system/user/logout",
    ];
    if ignore_paths.iter().any(|ignore_path| path.starts_with(ignore_path)) {
        return Ok(next.run(req).await);
    }

    let auth_header = req.headers().get(http::header::AUTHORIZATION).and_then(|header| header.to_str().ok());
    if auth_header.is_none() {
        let json = Json(BaseResponse {
            msg: "用户未认证，请求头缺少字段[Authorization]".to_string(),
            code: 401,
            data: Some("None".to_string()),
        });
        return Ok((StatusCode::UNAUTHORIZED, json).into_response());
    }

    let authorization = auth_header.unwrap();
    let token = authorization.to_string().replace("Bearer ", "");
    let jwt_token_error = JwtToken::verify(jwt_util::JWT_SECRET, &token);
    let jwt_token = match jwt_token_error {
        Ok(data) => data,
        Err(err) => {
            let er = match err {
                AppError::JwtTokenError(s) => s,
                _ => "no math error".to_string(),
            };
            let json = Json(BaseResponse {
                msg: er,
                code: 401,
                data: Some("None".to_string()),
            });
            return Ok((StatusCode::UNAUTHORIZED, json).into_response());
        }
    };

    fn has_permission(permissions: &[String], path: &str) -> bool {
        permissions.iter().any(|permission| {
            permission.strip_prefix("/api").unwrap_or(permission) == path
        })
    }

    match validate_and_get_user_info(&state.redis, jwt_token.id).await {
        Ok((user_id, permissions, token_1, is_admin)) => {
            if token_1 != token {
                let json = Json(BaseResponse {
                    msg: "无效的token".to_string(),
                    code: 401,
                    data: Some("None".to_string()),
                });
                return Ok((StatusCode::UNAUTHORIZED, json).into_response());
            }
            if is_admin || has_permission(&permissions, path) {
                req.headers_mut().insert("user_id", user_id.to_string().parse().unwrap());
                req.extensions_mut().insert(permissions);
                Ok(next.run(req).await)
            } else {
                let json = Json(BaseResponse {
                    msg: format!("用户未授权访问url:{}", path),
                    code: 401,
                    data: Some("None".to_string()),
                });
                Ok((StatusCode::UNAUTHORIZED, json).into_response())
            }
        }
        Err(e) => {
            let json = Json(BaseResponse {
                msg: e.to_string(),
                code: 401,
                data: Some("None".to_string()),
            });
            Ok((StatusCode::UNAUTHORIZED, json).into_response())
        }
    }
}

async fn validate_and_get_user_info(redis_client: &Client, user_id: i64) -> Result<(i64, Vec<String>, String, bool), AppError> {
    let mut conn = redis_client.get_connection().map_err(AppError::RedisError)?;

    // Fetch session fields from Redis via service method, which handles key existence and field retrieval
    let (permissions_str, token, is_admin) = SysUserService::fetch_session_info(&mut conn, user_id).await?;
    let permissions: Vec<String> = if permissions_str.is_empty() {
        Vec::new()
    } else {
        permissions_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };
    Ok((user_id, permissions, token, is_admin))
}
