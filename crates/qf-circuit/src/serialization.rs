use crate::circuit::Circuit;
use crate::error::CircuitError;

pub fn to_json(circuit: &Circuit) -> Result<String, CircuitError> {
    serde_json::to_string_pretty(circuit)
        .map_err(|e| CircuitError::SerializationError(e.to_string()))
}

pub fn from_json(json: &str) -> Result<Circuit, CircuitError> {
    serde_json::from_str(json).map_err(|e| CircuitError::SerializationError(e.to_string()))
}

pub fn to_yaml(circuit: &Circuit) -> Result<String, CircuitError> {
    serde_yaml::to_string(circuit).map_err(|e| CircuitError::SerializationError(e.to_string()))
}

pub fn from_yaml(yaml: &str) -> Result<Circuit, CircuitError> {
    serde_yaml::from_str(yaml).map_err(|e| CircuitError::SerializationError(e.to_string()))
}
