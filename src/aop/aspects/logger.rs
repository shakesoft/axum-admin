use std::any::Any;
use std::future::Future;
use std::sync::Arc;
use aspect_core::{Aspect, AspectError, AsyncAspect, AsyncJoinPoint, AsyncProceedingJoinPoint, JoinPoint, ProceedingJoinPoint};
use log::info;
use tokio::time::Instant;
use crate::AppState;
use crate::vo::system::sys_user_vo::QueryUserListReq;

#[derive(Default)]
pub struct Logger;
impl AsyncAspect for Logger {
    async fn before(&self, ctx: &AsyncJoinPoint) {
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

    async fn after(&self, _ctx: &AsyncJoinPoint, _result: &(dyn Any + Send + Sync))  {
        info!("Logger.after: function completed");
    }

    // async fn around(&self, pjp: AsyncProceedingJoinPoint<'_>) -> Result<Box<dyn Any + Send + Sync>, AspectError>  {
    //     let start = Instant::now();
    //     let function_name = pjp.context().function_name;
    //     let result = pjp.proceed().await;
    //     let elapsed = start.elapsed();
    //     println!("{} took {:?}", function_name, elapsed);
    //     match &result {
    //         Ok(val) => println!("{} executed successfully", function_name),
    //         Err(e) => println!("{} execution failed: {:?}", function_name, e),
    //     };
    //     result
    // }
}