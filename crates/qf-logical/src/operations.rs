use serde::{Deserialize, Serialize};

use crate::error::LogicalError;
use crate::qubit::{LogicalQubit, LogicalState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalGate {
    X,
    Z,
    H,
    CNOT,
    Measure,
}

/// Apply a logical gate (updates state tracking, no actual simulation)
pub fn apply_logical_gate(qubit: &mut LogicalQubit, gate: LogicalGate) -> Result<(), LogicalError> {
    match gate {
        LogicalGate::X => {
            qubit.state = match qubit.state {
                LogicalState::Zero => LogicalState::One,
                LogicalState::One => LogicalState::Zero,
                LogicalState::Plus => LogicalState::Plus,
                LogicalState::Minus => LogicalState::Minus,
                LogicalState::Unknown => LogicalState::Unknown,
            };
            Ok(())
        }
        LogicalGate::Z => {
            qubit.state = match qubit.state {
                LogicalState::Zero => LogicalState::Zero,
                LogicalState::One => LogicalState::One,
                LogicalState::Plus => LogicalState::Minus,
                LogicalState::Minus => LogicalState::Plus,
                LogicalState::Unknown => LogicalState::Unknown,
            };
            Ok(())
        }
        LogicalGate::H => {
            qubit.state = match qubit.state {
                LogicalState::Zero => LogicalState::Plus,
                LogicalState::One => LogicalState::Minus,
                LogicalState::Plus => LogicalState::Zero,
                LogicalState::Minus => LogicalState::One,
                LogicalState::Unknown => LogicalState::Unknown,
            };
            Ok(())
        }
        LogicalGate::CNOT => Err(LogicalError::UnsupportedOperation(
            "CNOT requires two logical qubits".to_string(),
        )),
        LogicalGate::Measure => {
            if qubit.state == LogicalState::Unknown
                || qubit.state == LogicalState::Plus
                || qubit.state == LogicalState::Minus
            {
                qubit.state = LogicalState::Unknown;
            }
            Ok(())
        }
    }
}

/// Apply logical CNOT between two qubits (simplified state tracking)
pub fn apply_logical_cnot(
    control: &mut LogicalQubit,
    target: &mut LogicalQubit,
) -> Result<(), LogicalError> {
    match (control.state, target.state) {
        (LogicalState::Zero, _) => {}
        (LogicalState::One, _) => {
            target.state = match target.state {
                LogicalState::Zero => LogicalState::One,
                LogicalState::One => LogicalState::Zero,
                other => other,
            };
        }
        _ => {
            control.state = LogicalState::Unknown;
            target.state = LogicalState::Unknown;
        }
    }
    Ok(())
}

/// Estimate resources for a logical operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalResourceEstimate {
    pub physical_qubits_per_logical: usize,
    pub total_physical_qubits: usize,
    pub num_logical_qubits: usize,
    pub code_distance: usize,
}

pub fn estimate_resources(
    num_logical_qubits: usize,
    distance: usize,
) -> Result<LogicalResourceEstimate, LogicalError> {
    let code = qf_qec_surface::SurfaceCode::new(distance)?;
    let phys_per_logical = code.total_qubits();
    Ok(LogicalResourceEstimate {
        physical_qubits_per_logical: phys_per_logical,
        total_physical_qubits: phys_per_logical * num_logical_qubits,
        num_logical_qubits,
        code_distance: distance,
    })
}
