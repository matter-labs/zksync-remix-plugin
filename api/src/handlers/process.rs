use crate::handlers::types::{ApiCommand, ApiCommandResult};
use crate::metrics::MethodLatency;
use crate::worker::{ProcessState, WorkerEngine};
use rocket::State;
use tracing::{info, instrument};
use uuid::Uuid;

#[instrument(skip(engine))]
#[get("/process_status/<process_id>")]
pub async fn get_process_status(process_id: String, engine: &State<WorkerEngine>) -> String {
    let _guard = MethodLatency::new("/process_status");
    info!("/process_status/{:?}", process_id);
    // get status of process by ID
    match Uuid::parse_str(&process_id) {
        Ok(process_uuid) => {
            if let Some(entry) = engine.arc_process_states.get(&process_uuid) {
                format!("{:}", entry.value())
            } else {
                "Process id not found".to_string()
            }
        }
        Err(e) => e.to_string(),
    }
}

pub fn do_process_command(command: ApiCommand, engine: &State<WorkerEngine>) -> String {
    // queue the new Scarb command
    match engine.enqueue_command(command) {
        Ok(uuid) => {
            format!("{}", uuid)
        }
        Err(e) => e,
    }
}

pub fn fetch_process_result<F>(
    process_id: String,
    engine: &State<WorkerEngine>,
    do_work: F,
) -> String
where
    F: FnOnce(&ApiCommandResult) -> String,
{
    // get status of process by ID
    match Uuid::parse_str(&process_id) {
        Ok(process_uuid) => {
            if engine.arc_process_states.contains_key(&process_uuid) {
                match engine
                    .arc_process_states
                    .get(&process_uuid)
                    .unwrap()
                    .value()
                {
                    ProcessState::Completed(result) => do_work(result),
                    _ => "Result not available".to_string(),
                }
            } else {
                "Process id not found".to_string()
            }
        }
        Err(e) => e.to_string(),
    }
}
