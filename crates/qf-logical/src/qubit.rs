use serde::{Deserialize, Serialize};

use crate::error::LogicalError;

/// Type of error-correcting code backing a logical qubit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeType {
    Surface { distance: usize },
    CssLdpc { name: String },
}

/// A logical qubit encoded in an error-correcting code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalQubit {
    pub id: usize,
    pub code_type: CodeType,
    pub num_physical_qubits: usize,
    pub state: LogicalState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalState {
    Zero,
    One,
    Plus,
    Minus,
    Unknown,
}

impl LogicalQubit {
    pub fn from_surface_code(id: usize, distance: usize) -> Result<Self, LogicalError> {
        let code = qf_qec_surface::SurfaceCode::new(distance)?;
        Ok(Self {
            id,
            code_type: CodeType::Surface { distance },
            num_physical_qubits: code.num_data_qubits(),
            state: LogicalState::Zero,
        })
    }

    pub fn from_css_code(id: usize, css: &qf_qec_ldpc::code::CssCode) -> Self {
        Self {
            id,
            code_type: CodeType::CssLdpc {
                name: css.name.clone(),
            },
            num_physical_qubits: css.num_physical_qubits(),
            state: LogicalState::Zero,
        }
    }

    pub fn physical_qubit_count(&self) -> usize {
        self.num_physical_qubits
    }
}
