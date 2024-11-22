#[macro_use]
extern crate rocket;

pub mod cors;
pub mod errors;
pub mod handlers;
mod metrics;
pub mod rate_limiter;
pub mod tracing_log;
pub mod utils;
pub mod worker;

use clokwerk::{Scheduler, TimeUnits};
use handlers::compile::{compile, compile_async, get_compile_result};
use handlers::compiler_version::{allowed_versions, compiler_version};
use handlers::process::get_process_status;
use handlers::utils::service_version;
use handlers::verify::{get_verify_result, verify, verify_async};
use handlers::{health, who_is_this};
use rocket::tokio::time::sleep;
use rocket::{tokio, Build, Rocket};
use std::env;
use std::time::Duration;
use tokio::sync::watch;
use tracing::info;
use vise_exporter::MetricsExporter;

use crate::cors::CORS;
use crate::errors::CoreError;
use crate::handlers::utils::on_plugin_launched;
use crate::rate_limiter::RateLimiter;
use crate::tracing_log::init_logger;
use crate::utils::lib::{ARTIFACTS_ROOT, SOL_ROOT};
use crate::worker::WorkerEngine;

async fn clear_artifacts() {
    let _ = tokio::fs::remove_dir_all(ARTIFACTS_ROOT).await;
    let _ = tokio::fs::create_dir_all(ARTIFACTS_ROOT).await;
    let _ = tokio::fs::remove_dir_all(SOL_ROOT).await;
    let _ = tokio::fs::create_dir_all(SOL_ROOT).await;
    info!("artifacts cleared!");
}

fn create_app() -> Rocket<Build> {
    const DEFAULT_NUM_OF_WORKERS: u32 = 2u32;
    const DEFAULT_QUEUE_SIZE: usize = 1_000;

    let number_of_workers = match env::var("WORKER_THREADS") {
        Ok(v) => v.parse::<u32>().unwrap_or(DEFAULT_NUM_OF_WORKERS),
        Err(_) => DEFAULT_NUM_OF_WORKERS,
    };

    let queue_size = match env::var("QUEUE_SIZE") {
        Ok(v) => v.parse::<usize>().unwrap_or(DEFAULT_QUEUE_SIZE),
        Err(_) => DEFAULT_QUEUE_SIZE,
    };

    // Launch the worker processes
    let mut engine = WorkerEngine::new(number_of_workers, queue_size);
    engine.start();

    // Create a new scheduler
    let mut scheduler = Scheduler::new();

    scheduler.every(1.day()).run(move || {
        tokio::spawn(async {
            clear_artifacts().await;
        });
    });

    // Run the scheduler in a separate thread
    tokio::spawn(async move {
        loop {
            scheduler.run_pending();
            sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    info!("Number of workers: {}", number_of_workers);
    info!("Queue size: {}", queue_size);

    info!("Starting Rocket webserver...");

    rocket::build()
        .manage(engine)
        .manage(RateLimiter::new())
        .attach(CORS)
        .mount(
            "/",
            routes![
                compile,
                compile_async,
                get_compile_result,
                verify,
                verify_async,
                get_verify_result,
                compiler_version,
                get_process_status,
                allowed_versions,
                health,
                who_is_this,
                service_version,
                on_plugin_launched
            ],
        )
}

#[rocket::main]
async fn main() -> Result<(), CoreError> {
    init_logger()?;

    let (shutdown_sender, mut shutdown_receiver) = watch::channel(());
    let app = create_app();

    const DEFAULT_PORT: u16 = 8001;
    let metrics_port = match env::var("PROMETHEUS_PORT") {
        Ok(v) => v.parse::<u16>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };
    tracing::info!("Launching prometheus exporter on port: {metrics_port}");
    let exporter = MetricsExporter::default().with_graceful_shutdown(async move {
        shutdown_receiver.changed().await.ok();
    });
    let bind_address = format!("0.0.0.0:{metrics_port}").parse().unwrap();

    let app_future = app.launch();
    let metrics_future = tokio::spawn(exporter.start(bind_address));

    tokio::select! {
        res = app_future => {
            info!("Rocket webserver has been shut down");
            shutdown_sender.send(()).unwrap();
            res.map(drop)?;
        }
        res = metrics_future => {
            info!("Prometheus exporter has been shut down");
            let _ = res?;
        }
    }
    shutdown_sender.send(()).unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
