use utoipa::OpenApi;

/*
 *OpenAPI 文档汇总，由 handler/vo 目录自动罗列
 */
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::system::sys_dept_handler::add_sys_dept,
        crate::handler::system::sys_dept_handler::delete_sys_dept,
        crate::handler::system::sys_dept_handler::delete_sys_dept1,
        crate::handler::system::sys_dept_handler::update_sys_dept,
        crate::handler::system::sys_dept_handler::update_sys_dept_status,
        crate::handler::system::sys_dept_handler::query_sys_dept_detail,
        crate::handler::system::sys_dept_handler::query_sys_dept_list,
        crate::handler::system::sys_dict_data_handler::add_sys_dict_data,
        crate::handler::system::sys_dict_data_handler::delete_sys_dict_data,
        crate::handler::system::sys_dict_data_handler::update_sys_dict_data,
        crate::handler::system::sys_dict_data_handler::update_sys_dict_data_status,
        crate::handler::system::sys_dict_data_handler::query_sys_dict_data_detail,
        crate::handler::system::sys_dict_data_handler::query_sys_dict_data_list,
        crate::handler::system::sys_dict_type_handler::add_sys_dict_type,
        crate::handler::system::sys_dict_type_handler::delete_sys_dict_type,
        crate::handler::system::sys_dict_type_handler::update_sys_dict_type,
        crate::handler::system::sys_dict_type_handler::update_sys_dict_type_status,
        crate::handler::system::sys_dict_type_handler::query_sys_dict_type_detail,
        crate::handler::system::sys_dict_type_handler::query_sys_dict_type_list,
        crate::handler::system::sys_login_log_handler::delete_sys_login_log,
        crate::handler::system::sys_login_log_handler::clean_sys_login_log,
        crate::handler::system::sys_login_log_handler::query_sys_login_log_detail,
        crate::handler::system::sys_login_log_handler::query_sys_login_log_list,
        crate::handler::system::sys_menu_handler::add_sys_menu,
        crate::handler::system::sys_menu_handler::delete_sys_menu,
        crate::handler::system::sys_menu_handler::update_sys_menu,
        crate::handler::system::sys_menu_handler::update_sys_menu_status,
        crate::handler::system::sys_menu_handler::query_sys_menu_detail,
        crate::handler::system::sys_menu_handler::query_sys_menu_list,
        crate::handler::system::sys_menu_handler::query_sys_menu_list_simple,
        crate::handler::system::sys_notice_handler::add_sys_notice,
        crate::handler::system::sys_notice_handler::delete_sys_notice,
        crate::handler::system::sys_notice_handler::update_sys_notice,
        crate::handler::system::sys_notice_handler::update_sys_notice_status,
        crate::handler::system::sys_notice_handler::query_sys_notice_detail,
        crate::handler::system::sys_notice_handler::query_sys_notice_list,
        crate::handler::system::sys_notice_handler::query_sys_notice_request,
        crate::handler::system::sys_operate_log_handler::delete_sys_operate_log,
        crate::handler::system::sys_operate_log_handler::clean_sys_operate_log,
        crate::handler::system::sys_operate_log_handler::query_sys_operate_log_detail,
        crate::handler::system::sys_operate_log_handler::query_sys_operate_log_list,
        crate::handler::system::sys_post_handler::add_sys_post,
        crate::handler::system::sys_post_handler::delete_sys_post,
        crate::handler::system::sys_post_handler::update_sys_post,
        crate::handler::system::sys_post_handler::update_sys_post_status,
        crate::handler::system::sys_post_handler::query_sys_post_detail,
        crate::handler::system::sys_post_handler::query_sys_post_list,
        crate::handler::system::sys_role_handler::add_sys_role,
        crate::handler::system::sys_role_handler::delete_sys_role,
        crate::handler::system::sys_role_handler::update_sys_role,
        crate::handler::system::sys_role_handler::update_sys_role_status,
        crate::handler::system::sys_role_handler::query_sys_role_detail,
        crate::handler::system::sys_role_handler::query_sys_role_list,
        crate::handler::system::sys_role_handler::query_role_menu,
        crate::handler::system::sys_role_handler::update_role_menu,
        crate::handler::system::sys_role_handler::query_allocated_list,
        crate::handler::system::sys_role_handler::query_unallocated_list,
        crate::handler::system::sys_role_handler::cancel_auth_user,
        crate::handler::system::sys_role_handler::batch_cancel_auth_user,
        crate::handler::system::sys_role_handler::batch_auth_user,
        crate::handler::system::sys_user_handler::add_sys_user,
        crate::handler::system::sys_user_handler::delete_sys_user,
        crate::handler::system::sys_user_handler::update_sys_user,
        crate::handler::system::sys_user_handler::update_sys_user_status,
        crate::handler::system::sys_user_handler::reset_sys_user_password,
        crate::handler::system::sys_user_handler::update_sys_user_password,
        crate::handler::system::sys_user_handler::query_sys_user_detail,
        crate::handler::system::sys_user_handler::query_sys_user_list,
        crate::handler::system::sys_user_handler::login,
        crate::handler::system::sys_user_handler::query_user_role,
        crate::handler::system::sys_user_handler::update_user_role,
        crate::handler::system::sys_user_handler::query_user_menu
    ),
    components(schemas(
        utoipa::TupleUnit,
        crate::vo::system::sys_user_vo::UserSession,
    )),
    tags((name = "axum-admin", description = "OpenAPI"))
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    // 生成的文档里不能有指向未注册 schema 的悬空 $ref
    #[test]
    fn no_dangling_refs() {
        let doc = serde_json::to_string(&ApiDoc::openapi()).unwrap();
        let known: Vec<String> = ApiDoc::openapi().components.unwrap().schemas.keys().cloned().collect();
        let mut dangling: Vec<&str> = doc
            .match_indices("#/components/schemas/")
            .map(|(i, m)| {
                let rest = &doc[i + m.len()..];
                &rest[..rest.find('"').unwrap()]
            })
            .filter(|name| !known.iter().any(|k| k == name))
            .collect();
        dangling.sort();
        dangling.dedup();
        assert!(dangling.is_empty(), "未注册的 schema: {:?}", dangling);
        assert_eq!(ApiDoc::openapi().paths.paths.len(), 72);
    }
}
