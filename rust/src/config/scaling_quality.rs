use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
use std::default::Default;
use serde::Deserialize;
use serde::Serialize;

/// Enum used to specify scaling quality for scaling up graphics
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum ScalingQuality {
    /// Use linear interpolation
    LINEAR,
    /// Use nearest neighbor interpolation
    NEAR_PERFECT,
    /// Scale up to the nearest multiple of 640x480 and use nearest neighbor interpolation
    PERFECT,
}

impl FromStr for ScalingQuality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LINEAR" => Ok(ScalingQuality::LINEAR),
            "NEAR_PERFECT" => Ok(ScalingQuality::NEAR_PERFECT),
            "PERFECT" => Ok(ScalingQuality::PERFECT),
            _ => Err(format!("Scaling quality {} is unknown", s))
        }
    }
}

impl Display for ScalingQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ScalingQuality::LINEAR => "Linear Interpolation",
            ScalingQuality::NEAR_PERFECT => "Near perfect with oversampling",
            ScalingQuality::PERFECT => "Pixel perfect centered",
        })
    }
}

impl Default for ScalingQuality {
    fn default() -> ScalingQuality {
        ScalingQuality::PERFECT
    }
}