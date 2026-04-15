use crate::common::result::BaseResponse;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use redis::RedisError;
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Serialize)]
struct ValidationErrorItem {
    message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidationErrorData {
    validation_errors: Vec<ValidationErrorItem>,
}

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Failed to complete an HTTP request")]
    // Http { #[from] source: reqwest::Error },
    #[error("Failed to read the cache file")]
    DiskCacheRead { source: std::io::Error },
    
    // #[error("Failed to update the cache file")]
    // DiskCacheWrite { source: std::io::Error },
    #[error("jwt：{0}")]
    JwtTokenError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),

    #[error("redis错误: {0}")]
    RedisError(#[from] RedisError),

    #[error("业务异常: {0}")]
    BusinessError(&'static str),

    #[error("验证异常")]
    ValidationError(Vec<String>),

    #[error("内部异常: {0}")]
    InternalError(&'static str),
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let errors = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|error| {
                    if let Some(message) = &error.message {
                        message.clone().into_owned()
                    } else {
                        "Invalid value".to_string()
                    }
                })
            })
            .collect::<Vec<String>>();
        AppError::ValidationError(errors)
    }
}

impl From<JsonRejection> for AppError {
    fn from(error: JsonRejection) -> Self {
        AppError::ValidationError(vec![error.body_text()])
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DbError(error) => {
                let response = BaseResponse {
                    msg: AppError::DbError(error).to_string(),
                    code: 1,
                    data: Some("None".to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            },
            AppError::RedisError(error) => {
                let response = BaseResponse {
                    msg: AppError::RedisError(error).to_string(),
                    code: 1,
                    data: Some("None".to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            },
            AppError::DiskCacheRead { source } => {
                let response = BaseResponse {
                    msg: AppError::DiskCacheRead { source }.to_string(),
                    code: 1,
                    data: Some("None".to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            },
            AppError::JwtTokenError(msg) => {
                let response = BaseResponse {
                    msg: AppError::JwtTokenError(msg).to_string(),
                    code: 1,
                    data: Some("None".to_string()),
                };
                (StatusCode::UNAUTHORIZED, Json(response)).into_response()
            },
            AppError::BusinessError(msg) => {
                let response = BaseResponse {
                    msg: AppError::BusinessError(msg).to_string(),
                    code: 1,
                    data: Some("None".to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            },
            AppError::ValidationError(messages) => {
                let response = BaseResponse {
                    msg: "参数校验失败".to_string(),
                    code: 1,
                    data: Some(ValidationErrorData {
                        validation_errors: messages
                            .into_iter()
                            .map(|message| ValidationErrorItem { message })
                            .collect(),
                    }),
                };
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            },
            AppError::InternalError(msg) => {
                let response = BaseResponse {
                    msg: AppError::InternalError(msg).to_string(),
                    code: 1,
                    data: Some("None".to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            },
        }
    }
}


impl AppError {
    pub fn default() -> AppError {
        AppError::InternalError("服务器发生内部异常，请稍后再试")
    }
    pub fn interrupt() -> AppResult<Json<BaseResponse<()>>> {
        Err(AppError::default())
    }
}
