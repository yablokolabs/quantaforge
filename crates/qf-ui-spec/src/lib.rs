use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCircuitRequest {
    pub num_qubits: usize,
    pub gates: Vec<GateSpec>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GateSpec {
    pub gate: String,
    pub qubits: Vec<usize>,
    pub parameter: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CircuitResponse {
    pub id: String,
    pub num_qubits: usize,
    pub gate_count: usize,
    pub depth: usize,
    pub circuit_json: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubmitJobRequest {
    pub circuit_id: String,
    pub shots: Option<usize>,
    pub noise_model: Option<NoiseModelSpec>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NoiseModelSpec {
    pub noise_type: String,
    pub parameter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JobResponse {
    pub id: String,
    pub status: String,
    pub result: Option<JobResultResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JobResultResponse {
    pub counts: HashMap<String, usize>,
    pub num_shots: usize,
    pub num_qubits: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub detail: Option<String>,
}
