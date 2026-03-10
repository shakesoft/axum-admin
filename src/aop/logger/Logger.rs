use aspect_core::{
    Aspect, AspectError, AsyncAspect, AsyncJoinPoint, JoinPoint, ProceedingJoinPoint,
};
use log::info;
use crate::vo::system::sys_user_vo::QueryUserListReq;

#[derive(Default)]
pub struct Logger;
impl Aspect for Logger {
    fn before(&self, ctx: &JoinPoint) {
        let arg1 = ctx.args.get(1).unwrap().downcast_ref::<QueryUserListReq>();
        println!("{}",arg1.unwrap().page_no);
        info!("{}: {},{},{},{}", ctx.function_name, ctx.module_path, ctx.location.file, ctx.location.line, ctx.args.iter().count());
    }
}


#[derive(Default)]
pub struct Logger1;
impl AsyncAspect for Logger1 {
    async fn before(&self, ctx: &AsyncJoinPoint)  {
        let arg1 = ctx.args.get(0).unwrap().downcast_ref::<i32>().unwrap();
        let arg2 = ctx.args.get(1).unwrap().downcast_ref::<i32>().unwrap();
        info!("{arg1:?} {arg2:?}");
        // info!("{}: {},{},{},{}", ctx.function_name, ctx.module_path, ctx.location.file, ctx.location.line, ctx.args.iter().count());
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
