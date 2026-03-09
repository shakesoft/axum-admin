use std::any::Any;
use aspect_core::{Aspect, AspectError, ProceedingJoinPoint};
use tokio::time::Instant;
use log::info;

#[derive(Default)]
pub struct Timer;

impl Aspect for Timer {
    // fn around(&self, pjp: ProceedingJoinPoint) -> Result<Box<dyn Any>, AspectError> {
    //     let start = Instant::now();
    //     let ctx = pjp.context();
    //     let function_name = ctx.function_name;
    //
    //     // Iterate over arguments and try to downcast to known types
    //     for (i, arg) in ctx.args.iter().enumerate() {
    //         let any_ref: &dyn Any = arg.as_ref();
    //         if let Some(s) = any_ref.downcast_ref::<String>() {
    //             info!("{} arg[{}]: String = {}", function_name, i, s);
    //         } else if let Some(n) = any_ref.downcast_ref::<i32>() {
    //             info!("{} arg[{}]: i32 = {}", function_name, i, n);
    //         } else if let Some(f) = any_ref.downcast_ref::<f64>() {
    //             info!("{} arg[{}]: f64 = {}", function_name, i, f);
    //         } else if let Some(b) = any_ref.downcast_ref::<bool>() {
    //             info!("{} arg[{}]: bool = {}", function_name, i, b);
    //         } else {
    //             info!("{} arg[{}]: unknown type = {:?}", function_name, i, any_ref.type_id());
    //         }
    //     }
    //
    //     let result = pjp.proceed();
    //     let elapsed = start.elapsed();
    //     info!("{} took {:?}", function_name, elapsed);
    //     result
    // }

    fn around(&self, pjp: ProceedingJoinPoint) -> Result<Box<dyn Any>, AspectError>{
        let start = Instant::now();
        let function_name = pjp.context().function_name;
        let result = pjp.proceed();
        let elapsed = start.elapsed();
        info!("{} took {:?}", function_name, elapsed);
        result
    }

    // fn around(&self, pjp: ProceedingJoinPoint) ->  Result<Box<dyn Any>, AspectError>  {
    //     let start = Instant::now();
    //     let function_name = pjp.context().function_name;
    //     let result = pjp.proceed();
    //     let elapsed = start.elapsed();
    //     info!("{} took {:?}", function_name, elapsed);
    //     info!("{} took {:?}", function_name, elapsed);
    //     result
    // }
}
