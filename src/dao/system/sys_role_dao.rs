use rbatis::RBatis;
use rbs::value;

pub struct SysRoleDao;

impl SysRoleDao {
    // 封装更新角色状态的数据库操作
    pub async fn update_status(rb: &RBatis, ids: &Vec<i64>, status: i8) -> rbatis::Result<()> {
        let update_sql = format!("update sys_role set status = ? where id in ({})", ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }

    // 封装批量取消用户授权的数据库操作
    pub async fn batch_cancel_auth_user(rb: &RBatis, role_id: i64, user_ids: &Vec<i64>) -> rbatis::Result<()> {
        let update_sql = format!(
            "delete from sys_user_role where role_id = ? and user_id in ({})",
            user_ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(role_id)];
        param.extend(user_ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }
}