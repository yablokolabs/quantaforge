use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use qf_circuit::serialization;
use qf_sim_statevec::StateVectorSimulator;
use qf_ui_spec::{ErrorResponse, JobResponse, JobResultResponse, SubmitJobRequest};
use uuid::Uuid;

use crate::state::{SharedState, StoredJob};

pub async fn submit_job(
    State(state): State<SharedState>,
    Json(req): Json<SubmitJobRequest>,
) -> Result<Json<JobResponse>, (StatusCode, Json<ErrorResponse>)> {
    let shots = req.shots.unwrap_or(1024);

    // Look up the circuit
    let circuit_json = {
        let circuits = state.circuits.read().await;
        match circuits.get(&req.circuit_id) {
            Some(stored) => stored.circuit_json.clone(),
            None => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "Circuit not found".to_string(),
                        detail: Some(format!("No circuit with id: {}", req.circuit_id)),
                    }),
                ));
            }
        }
    };

    let job_id = Uuid::new_v4().to_string();

    // Create job entry with status "queued"
    {
        let mut jobs = state.jobs.write().await;
        jobs.insert(
            job_id.clone(),
            StoredJob {
                id: job_id.clone(),
                circuit_id: req.circuit_id.clone(),
                shots,
                status: "queued".to_string(),
                result: None,
            },
        );
    }

    // Deserialize and run simulation
    let circuit = serialization::from_json(&circuit_json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to deserialize circuit".to_string(),
                detail: Some(e.to_string()),
            }),
        )
    })?;

    let sim = StateVectorSimulator::new();
    let shot_result = sim.run_shots(&circuit, shots).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Simulation failed".to_string(),
                detail: Some(e.to_string()),
            }),
        )
    })?;

    let result = JobResultResponse {
        counts: shot_result.counts,
        num_shots: shot_result.num_shots,
        num_qubits: shot_result.num_qubits,
    };

    // Update job status to "completed"
    {
        let mut jobs = state.jobs.write().await;
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = "completed".to_string();
            job.result = Some(result.clone());
        }
    }

    Ok(Json(JobResponse {
        id: job_id,
        status: "completed".to_string(),
        result: Some(result),
    }))
}

pub async fn get_job(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<JobResponse>, (StatusCode, Json<ErrorResponse>)> {
    let jobs = state.jobs.read().await;
    match jobs.get(&id) {
        Some(stored) => Ok(Json(JobResponse {
            id: stored.id.clone(),
            status: stored.status.clone(),
            result: stored.result.clone(),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Job not found".to_string(),
                detail: Some(format!("No job with id: {id}")),
            }),
        )),
    }
}

pub async fn list_jobs(State(state): State<SharedState>) -> Json<Vec<JobResponse>> {
    let jobs = state.jobs.read().await;
    let list: Vec<JobResponse> = jobs
        .values()
        .map(|j| JobResponse {
            id: j.id.clone(),
            status: j.status.clone(),
            result: j.result.clone(),
        })
        .collect();
    Json(list)
}
