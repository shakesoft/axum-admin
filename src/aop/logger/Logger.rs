use std::sync::Arc;
use aspect_core::{
    Aspect, JoinPoint,
};
use log::info;
use crate::AppState;
use crate::vo::system::sys_user_vo::QueryUserListReq;

#[derive(Default)]
pub struct Logger;
impl Aspect for Logger {
    fn before(&self, ctx: &JoinPoint) {
        // Safely try to get the first argument as Arc<AppState> and log presence instead of attempting to format non-Display types
        let arg0 = ctx.args.get(0).and_then(|b| b.downcast_ref::<Arc<AppState>>());
        if let Some(app_state) = arg0 {
            info!("Logger.before: received AppState (rbatis pool present)");
            // if you need to inspect more, do it via methods that return Display/Debug values
        } else {
            info!("Logger.before: arg0 missing or not Arc<AppState>");
        }

        // Safely try to get the second argument as QueryUserListReq and log its page_no
        let arg1 = ctx.args.get(1).and_then(|b| b.downcast_ref::<QueryUserListReq>());
        if let Some(q) = arg1 {
            info!("Logger.before: page_no = {}", q.page_no);
        } else {
            info!("Logger.before: arg1 missing or not QueryUserListReq");
        }

        info!("{}: {},{},{},{}", ctx.function_name, ctx.module_path, ctx.location.file, ctx.location.line, ctx.args.iter().count());
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use aspect_core::joinpoint::{JoinPoint, Location};

    #[test]
    fn logger_before_inspection() {
        let _ = env_logger::builder().is_test(true).try_init();

        let jp = JoinPoint::new(
            "add",
            "crate::handler::system::sys_user_handler",
            Location { file: "src/handler/system/sys_user_handler.rs", line: 1 },
            vec![Box::new("1".to_string()), Box::new("2".to_string())],
        );

        let logger = Logger::default();
        logger.before(&jp);
    }
}
