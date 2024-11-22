pub mod compile;
pub mod compiler_version;
pub mod process;
pub mod types;
pub mod utils;
pub mod verify;

use lazy_static::lazy_static;
use tokio::sync::Semaphore;
use tracing::info;
use tracing::instrument;

use crate::errors::ApiError;
use crate::handlers::compile::do_compile;
use crate::handlers::compiler_version::do_compiler_version;
use crate::handlers::types::{ApiCommand, ApiCommandResult, HealthCheckResponse};
use crate::handlers::verify::do_verify;
use crate::metrics::MethodLatency;

const PROCESS_SPAWN_LIMIT: usize = 8;
lazy_static! {
    static ref SPAWN_SEMAPHORE: Semaphore = Semaphore::new(PROCESS_SPAWN_LIMIT);
}

#[instrument]
#[get("/health")]
pub async fn health() -> HealthCheckResponse {
    let _guard = MethodLatency::new("/health");
    info!("/health");
    HealthCheckResponse::ok()
}

#[instrument]
#[get("/")]
pub async fn who_is_this() -> &'static str {
    info!("/who_is_this");
    "Who are you?"
}

pub async fn dispatch_command(command: ApiCommand) -> Result<ApiCommandResult, ApiError> {
    match command {
        ApiCommand::CompilerVersion => match do_compiler_version() {
            Ok(result) => Ok(ApiCommandResult::CompilerVersion(result)),
            Err(e) => Err(e),
        },
        ApiCommand::Compile(request) => match do_compile(request).await {
            Ok(compile_response) => Ok(ApiCommandResult::Compile(compile_response.into_inner())),
            Err(e) => Err(e),
        },
        ApiCommand::Verify(request) => match do_verify(request).await {
            Ok(verify_response) => Ok(ApiCommandResult::Verify(verify_response.into_inner())),
            Err(e) => Err(e),
        },
        ApiCommand::Shutdown => Ok(ApiCommandResult::Shutdown),
    }
}
