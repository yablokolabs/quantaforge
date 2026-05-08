use serde::{Deserialize, Serialize};

use crate::amplitude_damping::AmplitudeDampingChannel;
use crate::channel::NoiseChannel;
use crate::depolarizing::DepolarizingChannel;
use crate::error::NoiseError;
use crate::phase_damping::PhaseDampingChannel;
use crate::readout::ReadoutNoise;

/// Specification for a coherent noise channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoiseSpec {
    Depolarizing { probability: f64 },
    AmplitudeDamping { gamma: f64 },
    PhaseDamping { gamma: f64 },
}

/// Specification for classical readout noise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadoutNoiseSpec {
    pub p0_to_1: f64,
    pub p1_to_0: f64,
}

/// A complete noise model combining gate noise, readout noise, and idle noise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseModel {
    pub gate_noise: Option<NoiseSpec>,
    pub readout_noise: Option<ReadoutNoiseSpec>,
    pub idle_noise: Option<NoiseSpec>,
}

impl NoiseModel {
    /// An ideal (noiseless) model.
    pub fn ideal() -> Self {
        Self {
            gate_noise: None,
            readout_noise: None,
            idle_noise: None,
        }
    }

    /// Convenience: create a model with only depolarizing gate noise.
    pub fn with_depolarizing(p: f64) -> Result<Self, NoiseError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(NoiseError::InvalidParameter {
                param: "probability",
                value: p,
            });
        }
        Ok(Self {
            gate_noise: Some(NoiseSpec::Depolarizing { probability: p }),
            readout_noise: None,
            idle_noise: None,
        })
    }

    /// Validate that all parameters are in range. Call after deserialization.
    pub fn validate(&self) -> Result<(), NoiseError> {
        if let Some(spec) = &self.gate_noise {
            validate_spec(spec)?;
        }
        if let Some(spec) = &self.idle_noise {
            validate_spec(spec)?;
        }
        if let Some(ro) = &self.readout_noise {
            validate_probability("p0_to_1", ro.p0_to_1)?;
            validate_probability("p1_to_0", ro.p1_to_0)?;
        }
        Ok(())
    }

    /// Build a boxed NoiseChannel from the gate noise spec, if present.
    pub fn gate_channel(&self) -> Result<Option<Box<dyn NoiseChannel>>, NoiseError> {
        self.gate_noise.as_ref().map(build_channel).transpose()
    }

    /// Build a ReadoutNoise from the readout noise spec, if present.
    pub fn readout(&self) -> Result<Option<ReadoutNoise>, NoiseError> {
        self.readout_noise
            .as_ref()
            .map(|spec| ReadoutNoise::new(spec.p0_to_1, spec.p1_to_0))
            .transpose()
    }
}

fn validate_probability(param: &'static str, value: f64) -> Result<(), NoiseError> {
    if !(0.0..=1.0).contains(&value) {
        return Err(NoiseError::InvalidParameter { param, value });
    }
    Ok(())
}

fn validate_spec(spec: &NoiseSpec) -> Result<(), NoiseError> {
    match spec {
        NoiseSpec::Depolarizing { probability } => {
            validate_probability("probability", *probability)
        }
        NoiseSpec::AmplitudeDamping { gamma } => validate_probability("gamma", *gamma),
        NoiseSpec::PhaseDamping { gamma } => validate_probability("gamma", *gamma),
    }
}

fn build_channel(spec: &NoiseSpec) -> Result<Box<dyn NoiseChannel>, NoiseError> {
    match spec {
        NoiseSpec::Depolarizing { probability } => {
            Ok(Box::new(DepolarizingChannel::new(*probability)?))
        }
        NoiseSpec::AmplitudeDamping { gamma } => {
            Ok(Box::new(AmplitudeDampingChannel::new(*gamma)?))
        }
        NoiseSpec::PhaseDamping { gamma } => Ok(Box::new(PhaseDampingChannel::new(*gamma)?)),
    }
}
