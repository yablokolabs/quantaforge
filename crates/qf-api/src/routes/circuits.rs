use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use qf_circuit::serialization;
use qf_circuit::{CircuitBuilder, Gate, Instruction};
use qf_ui_spec::{CircuitResponse, CreateCircuitRequest, ErrorResponse};
use uuid::Uuid;

use crate::state::{SharedState, StoredCircuit};

fn build_gate(
    name: &str,
    qubits: &[usize],
    parameter: Option<f64>,
) -> Result<(Gate, Vec<usize>), String> {
    match name.to_uppercase().as_str() {
        "H" => Ok((Gate::H, qubits.to_vec())),
        "X" => Ok((Gate::X, qubits.to_vec())),
        "Y" => Ok((Gate::Y, qubits.to_vec())),
        "Z" => Ok((Gate::Z, qubits.to_vec())),
        "S" => Ok((Gate::S, qubits.to_vec())),
        "T" => Ok((Gate::T, qubits.to_vec())),
        "RX" => {
            let theta = parameter.ok_or("RX requires a parameter")?;
            Ok((Gate::Rx(theta), qubits.to_vec()))
        }
        "RY" => {
            let theta = parameter.ok_or("RY requires a parameter")?;
            Ok((Gate::Ry(theta), qubits.to_vec()))
        }
        "RZ" => {
            let theta = parameter.ok_or("RZ requires a parameter")?;
            Ok((Gate::Rz(theta), qubits.to_vec()))
        }
        "CNOT" | "CX" => Ok((Gate::CNOT, qubits.to_vec())),
        "CZ" => Ok((Gate::CZ, qubits.to_vec())),
        "MEASURE" => Ok((Gate::Measure, qubits.to_vec())),
        other => Err(format!("Unknown gate: {other}")),
    }
}

pub async fn create_circuit(
    State(state): State<SharedState>,
    Json(req): Json<CreateCircuitRequest>,
) -> Result<Json<CircuitResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut builder = CircuitBuilder::new(req.num_qubits).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid circuit".to_string(),
                detail: Some(e.to_string()),
            }),
        )
    })?;

    for gs in &req.gates {
        let (gate, qubits) = build_gate(&gs.gate, &gs.qubits, gs.parameter).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid gate".to_string(),
                    detail: Some(e),
                }),
            )
        })?;
        let instr = Instruction::new(gate, qubits).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid instruction".to_string(),
                    detail: Some(e.to_string()),
                }),
            )
        })?;
        builder = builder_add_instruction(builder, instr).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Failed to add instruction".to_string(),
                    detail: Some(e.to_string()),
                }),
            )
        })?;
    }

    let circuit = builder.build();
    let circuit_json = serialization::to_json(&circuit).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Serialization failed".to_string(),
                detail: Some(e.to_string()),
            }),
        )
    })?;

    let id = Uuid::new_v4().to_string();
    let response = CircuitResponse {
        id: id.clone(),
        num_qubits: circuit.num_qubits(),
        gate_count: circuit.gate_count(),
        depth: circuit.depth(),
        circuit_json: circuit_json.clone(),
    };

    let stored = StoredCircuit {
        id: id.clone(),
        circuit_json,
        num_qubits: circuit.num_qubits(),
        gate_count: circuit.gate_count(),
        depth: circuit.depth(),
    };
    state.circuits.write().await.insert(id, stored);

    Ok(Json(response))
}

/// Helper: CircuitBuilder consumes self, so we can't use a simple loop with `&mut`.
/// We rebuild via raw circuit instruction addition.
fn builder_add_instruction(
    builder: CircuitBuilder,
    instr: Instruction,
) -> Result<CircuitBuilder, qf_circuit::CircuitError> {
    let gate = &instr.gate;
    let qubits = &instr.qubits;
    match gate {
        Gate::H => builder.h(qubits[0]),
        Gate::X => builder.x(qubits[0]),
        Gate::Y => builder.y(qubits[0]),
        Gate::Z => builder.z(qubits[0]),
        Gate::S => builder.s(qubits[0]),
        Gate::T => builder.t(qubits[0]),
        Gate::Rx(theta) => builder.rx(qubits[0], *theta),
        Gate::Ry(theta) => builder.ry(qubits[0], *theta),
        Gate::Rz(theta) => builder.rz(qubits[0], *theta),
        Gate::CNOT => builder.cnot(qubits[0], qubits[1]),
        Gate::CZ => builder.cz(qubits[0], qubits[1]),
        Gate::Measure => builder.measure(qubits[0]),
    }
}

pub async fn get_circuit(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<CircuitResponse>, (StatusCode, Json<ErrorResponse>)> {
    let circuits = state.circuits.read().await;
    match circuits.get(&id) {
        Some(stored) => Ok(Json(CircuitResponse {
            id: stored.id.clone(),
            num_qubits: stored.num_qubits,
            gate_count: stored.gate_count,
            depth: stored.depth,
            circuit_json: stored.circuit_json.clone(),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Circuit not found".to_string(),
                detail: Some(format!("No circuit with id: {id}")),
            }),
        )),
    }
}
