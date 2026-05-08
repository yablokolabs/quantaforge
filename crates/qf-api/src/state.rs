use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct StoredCircuit {
    pub id: String,
    pub circuit_json: String,
    pub num_qubits: usize,
    pub gate_count: usize,
    pub depth: usize,
}

pub struct StoredJob {
    pub id: String,
    pub circuit_id: String,
    pub shots: usize,
    pub status: String,
    pub result: Option<qf_ui_spec::JobResultResponse>,
}

pub struct AppState {
    pub circuits: RwLock<HashMap<String, StoredCircuit>>,
    pub jobs: RwLock<HashMap<String, StoredJob>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            circuits: RwLock::new(HashMap::new()),
            jobs: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedState = Arc<AppState>;
