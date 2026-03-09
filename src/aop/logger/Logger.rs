use std::any::Any;
use std::error::Error;
use aspect_core::{Aspect, AspectError, JoinPoint, ProceedingJoinPoint};
use log::info;
use tokio::time::Instant;

#[derive(Default)]
pub struct Logger;

impl Aspect for Logger {
    fn before(&self, ctx: &JoinPoint) {
        info!("{}: {},{},{}",ctx.function_name,ctx.module_path,ctx.location.file,ctx.location.line);
    }
}