use crate::common::error::AppError;
use crate::model::system::sys_user_model::User;
use crate::service::system::sys_login_log_service::SysLoginLogService;
use crate::utils::{jwt_util, time_util};
use crate::utils::jwt_util::JwtToken;
use crate::vo::system::sys_user_vo::UserLoginResp;
use rbatis::RBatis;
use rbatis::rbdc::DateTime;
use chrono::Local;
use redis::Commands;
use crate::dao::system::sys_user_dao::SysUserDao;
use crate::utils::user_agent_util::UserAgentUtil;

pub struct SysUserService;

impl SysUserService {
	/// Called when a user successfully authenticates.
	///
	/// This will:
	/// - create a JWT token
	/// - store permission and session info into redis
	/// - write a successful login log
	/// - update the user's last login info in the database
	pub async fn on_successful_login(
		rb: &RBatis,
		conn: &mut redis::Connection,
		user: &mut User,
		mobile: String,
		agent: UserAgentUtil,
	) -> Result<UserLoginResp, AppError> {
		let user_id =user.id.unwrap();

		//判断用户菜单权限，获取用户权限列表
		let (btn_menu, is_super) = SysUserDao::query_btn_menu(rb, &user_id).await;
		if btn_menu.len() == 0 {
			SysLoginLogService::add_login_log(rb, mobile, 0, "用户没有分配角色或者菜单,不能登录", agent).await;
			return Err(AppError::BusinessError("用户没有分配角色或者菜单,不能登录"));
		}

		// generate token
		let jwt = JwtToken::new(user_id, &user.user_name);
		let expires_at = jwt.get_exp();
		let expires_time = time_util::timestamp_to_local(expires_at as i64);
		let token = jwt.create_token(jwt_util::JWT_SECRET)?;

		// persist session info to redis
		let key = format!("axum:admin:user:info:{:?}", &user_id);
		conn.hset::<_, _, _, ()>(&key, "permissions", &btn_menu.join(","))?; // permissions
		conn.hset::<_, _, _, ()>(&key, "user_name", &user.user_name)?; // user name
		conn.hset::<_, _, _, ()>(&key, "is_admin", is_super)?; // is super
		conn.hset::<_, _, _, ()>(&key, "token", &token)?; // token
		conn.hset::<_, _, _, ()>(&key, "last_login", Local::now().format("%Y-%m-%d %H:%M:%S").to_string())?;
		conn.hset::<_, _, _, ()>(&key, "expires_at", expires_time.unwrap_or_default().format("%Y-%m-%d %H:%M:%S").to_string())?;

		// record login log
		SysLoginLogService::add_login_log(rb, mobile, 1, "登录成功", agent.clone()).await;

		// update user last login info
		user.login_os = agent.os;
		user.login_browser = agent.browser;
		user.login_date = Some(DateTime::now());
		User::update_by_map(rb, &user, rbs::value! {"id": &user_id}).await?;

		Ok(UserLoginResp { token, expires_at })
	}

	/// Fetch raw session fields from redis for a user
	/// Returns (permissions_str, token, is_admin) or Err(AppError)
	pub async fn fetch_session_info(
		conn: &mut redis::Connection,
		user_id: i64,
	) -> Result<(String, String, bool), AppError> {
		let key = format!("axum:admin:user:info:{}", user_id);

		// check key existence first
		let exists: bool = conn.exists(&key).map_err(AppError::RedisError)?;
		if !exists {
			return Err(AppError::BusinessError("用户未登录"));
		}

		let permissions_str: String = conn.hget(&key, "permissions").unwrap_or_else(|_| "".to_string());
		let token: String = conn.hget(&key, "token").map_err(AppError::RedisError)?;
		let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();
		Ok((permissions_str, token, is_admin))
	}
}