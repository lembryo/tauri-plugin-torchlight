use serde::{Deserialize, Serialize};

/// Payload sent to the native mobile plugin for the `torch` command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorchRequest {
    /// `true` turns the torch on, `false` turns it off.
    pub enabled: bool,
    /// Optional brightness, in the `0.0..=1.0` range. `None` (or values when
    /// turning the torch off) means "use the platform default (maximum)".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
}

/// Response returned by the native `isAvailable` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub available: bool,
}

/// Response returned by the native `isEnabled` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnabledResponse {
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn torch_request_omits_level_when_absent() {
        let json = serde_json::to_string(&TorchRequest {
            enabled: true,
            level: None,
        })
        .unwrap();
        assert_eq!(json, r#"{"enabled":true}"#);
    }

    #[test]
    fn torch_request_serializes_level_with_camel_case() {
        let json = serde_json::to_string(&TorchRequest {
            enabled: true,
            level: Some(0.5),
        })
        .unwrap();
        assert_eq!(json, r#"{"enabled":true,"level":0.5}"#);
    }

    #[test]
    fn availability_response_round_trips() {
        let parsed: AvailabilityResponse =
            serde_json::from_str(r#"{"available":true}"#).unwrap();
        assert!(parsed.available);
    }

    #[test]
    fn enabled_response_round_trips() {
        let parsed: EnabledResponse = serde_json::from_str(r#"{"enabled":false}"#).unwrap();
        assert!(!parsed.enabled);
    }
}
