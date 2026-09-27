use serde::{Deserialize, Serialize};

/// Lowest brightness that still counts as "on".
///
/// Both natives treat a requested level of `0.0` as "on, as dim as this device
/// allows" rather than "off": Android clamps the mapped strength to at least 1,
/// and iOS floors the level at [`MIN_LEVEL`] because `setTorchModeOn(level:)`
/// raises `NSInvalidArgumentException` for exactly `0.0`.
pub const MIN_LEVEL: f64 = 0.01;

/// Normalizes a caller-supplied brightness.
///
/// Non-finite values (`NaN`, `±inf`) are treated as "no level requested", which
/// means full brightness. Finite values are clamped into `MIN_LEVEL..=1.0`.
pub(crate) fn sanitize_level(level: Option<f64>) -> Option<f64> {
    level
        .filter(|value| value.is_finite())
        .map(|value| value.clamp(MIN_LEVEL, 1.0))
}

/// Payload sent to the native mobile plugin for the `torch` command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorchRequest {
    /// `true` turns the torch on, `false` turns it off.
    pub enabled: bool,
    /// Optional brightness, in the `MIN_LEVEL..=1.0` range. `None` (or values
    /// when turning the torch off) means "use the platform default (maximum)".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
}

/// Payload sent to the native mobile plugin for the `toggle` command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleRequest {
    /// Optional brightness applied when the toggle turns the torch on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
}

/// Response returned by the native `isAvailable` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub available: bool,
}

/// Response returned by the native `isEnabled` and `toggle` commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnabledResponse {
    pub enabled: bool,
}

/// What the device can actually do with its torch.
///
/// This exists because `isAvailable` alone is not enough to drive a UI: it
/// reports whether the hardware exists, which never changes, while a torch can
/// be temporarily unusable, and brightness control is supported only on iOS and
/// on Android 13+ devices that report more than one strength level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorchCapabilities {
    /// Whether the device has a torch at all. Static for the life of the app.
    pub available: bool,
    /// Whether the torch can be used *right now*. Can flip at runtime when
    /// another app takes the camera or the device is under thermal duress.
    pub usable: bool,
    /// Whether a `level` passed to `torch`/`toggle` actually changes the
    /// brightness. When `false` the torch still turns on, at full power.
    pub brightness_supported: bool,
    /// Number of discrete brightness steps the device exposes, when it has a
    /// discrete scale (Android). `None` means a continuous scale (iOS) or no
    /// brightness control at all.
    pub level_steps: Option<u32>,
}

impl TorchCapabilities {
    /// The capabilities of a platform with no torch hardware.
    pub const NONE: Self = Self {
        available: false,
        usable: false,
        brightness_supported: false,
        level_steps: None,
    };
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
        let parsed: AvailabilityResponse = serde_json::from_str(r#"{"available":true}"#).unwrap();
        assert!(parsed.available);
    }

    #[test]
    fn enabled_response_round_trips() {
        let parsed: EnabledResponse = serde_json::from_str(r#"{"enabled":false}"#).unwrap();
        assert!(!parsed.enabled);
    }

    #[test]
    fn capabilities_use_camel_case_on_the_wire() {
        let parsed: TorchCapabilities = serde_json::from_str(
            r#"{"available":true,"usable":true,"brightnessSupported":true,"levelSteps":5}"#,
        )
        .unwrap();
        assert!(parsed.brightness_supported);
        assert_eq!(parsed.level_steps, Some(5));
    }

    #[test]
    fn capabilities_none_reports_nothing_available() {
        let json = serde_json::to_string(&TorchCapabilities::NONE).unwrap();
        assert_eq!(
            json,
            r#"{"available":false,"usable":false,"brightnessSupported":false,"levelSteps":null}"#
        );
    }

    #[test]
    fn non_finite_levels_are_treated_as_no_level() {
        assert_eq!(sanitize_level(Some(f64::NAN)), None);
        assert_eq!(sanitize_level(Some(f64::INFINITY)), None);
        assert_eq!(sanitize_level(Some(f64::NEG_INFINITY)), None);
        assert_eq!(sanitize_level(None), None);
    }

    #[test]
    fn levels_are_clamped_into_the_usable_range() {
        // 0.0 means "as dim as possible", never "off".
        assert_eq!(sanitize_level(Some(0.0)), Some(MIN_LEVEL));
        assert_eq!(sanitize_level(Some(-1.0)), Some(MIN_LEVEL));
        assert_eq!(sanitize_level(Some(2.0)), Some(1.0));
        assert_eq!(sanitize_level(Some(0.5)), Some(0.5));
    }
}
