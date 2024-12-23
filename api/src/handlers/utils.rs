use tracing::instrument;

use crate::metrics::MethodLatency;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[instrument]
#[get("/service_version")]
pub async fn service_version() -> String {
    let _guard = MethodLatency::new("/service_version");
    tracing::info!("/service_version");
    std::env::var("SERVICE_VERSION").unwrap_or_else(|_| format!("v{VERSION}"))
}

#[instrument]
#[post("/on-plugin-launched")]
pub async fn on_plugin_launched() {
    let _guard = MethodLatency::new("/on-plugin-launched");
    tracing::info!("/on-plugin-launched");
}
