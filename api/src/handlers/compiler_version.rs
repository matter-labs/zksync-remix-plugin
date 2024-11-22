use crate::utils::lib::ZKSOLC_VERSIONS;
use crate::{errors::ApiError, metrics::MethodLatency};
use rocket::serde::json::serde_json;
use tracing::{info, instrument};

#[instrument]
#[get("/compiler_version")]
pub async fn compiler_version() -> String {
    let _guard = MethodLatency::new("/compiler_version");
    info!("/compiler_version");
    do_compiler_version().unwrap_or_else(|e| e.to_string())
}

#[instrument]
#[get("/allowed_versions")]
pub async fn allowed_versions() -> String {
    let _guard = MethodLatency::new("/allowed_versions");
    info!("/allowed_versions");
    do_allowed_versions().unwrap_or_else(|e| format!("Error: {:?}", e))
}

/// Run ./zksolc --version to return compiler version string
///
pub fn do_compiler_version() -> Result<String, ApiError> {
    Ok("zksolc-latest".to_string())
}

pub fn do_allowed_versions() -> Result<String, ApiError> {
    Ok(serde_json::to_string(&ZKSOLC_VERSIONS).unwrap())
}
