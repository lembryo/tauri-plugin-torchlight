use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the torchlight commands.
///
/// The variants are platform-partitioned, so downstream Rust consumers that
/// match on this enum must either handle the `_` arm or gate their arms on
/// `cfg(mobile)` / `cfg(desktop)`. The enum is `#[non_exhaustive]` to make that
/// requirement explicit and to keep room for future variants.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The mobile plugin (Android/iOS) returned an error while running a command.
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),

    /// Torchlight has no hardware backing on desktop platforms.
    #[cfg(desktop)]
    #[error("torchlight is only supported on Android and iOS; this is a desktop platform")]
    Unsupported,
}

impl Error {
    /// A stable, machine-readable discriminator for this error.
    ///
    /// Callers should branch on this instead of matching on the human-readable
    /// message. The native plugins pass their own codes (`unavailable`,
    /// `cameraAccess`, `torchFailed`) through `Invoke.reject(message, code)`;
    /// anything without a code is reported as `internal`.
    pub fn kind(&self) -> String {
        match self {
            #[cfg(desktop)]
            Self::Unsupported => "unsupported".to_owned(),
            #[cfg(mobile)]
            Self::PluginInvoke(error) => match error {
                tauri::plugin::mobile::PluginInvokeError::InvokeRejected(response) => response
                    .code
                    .clone()
                    .unwrap_or_else(|| "torchFailed".to_owned()),
                _ => "internal".to_owned(),
            },
        }
    }

    /// The human-readable message, without the code prefix the mobile runtime
    /// adds to its `Display` output.
    pub fn message(&self) -> String {
        match self {
            #[cfg(desktop)]
            Self::Unsupported => self.to_string(),
            #[cfg(mobile)]
            Self::PluginInvoke(error) => match error {
                tauri::plugin::mobile::PluginInvokeError::InvokeRejected(response) => response
                    .message
                    .clone()
                    .unwrap_or_else(|| response.to_string()),
                other => other.to_string(),
            },
        }
    }
}

/// Serialized as `{ "kind": "...", "message": "..." }` so the JavaScript side can
/// branch on `kind` instead of pattern-matching English prose.
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TorchlightError", 2)?;
        state.serialize_field("kind", &self.kind())?;
        state.serialize_field("message", &self.message())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(desktop)]
    #[test]
    fn unsupported_serializes_with_kind_and_message() {
        let json = serde_json::to_string(&Error::Unsupported).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"unsupported","message":"torchlight is only supported on Android and iOS; this is a desktop platform"}"#
        );
    }

    #[cfg(desktop)]
    #[test]
    fn unsupported_reports_its_kind() {
        assert_eq!(Error::Unsupported.kind(), "unsupported");
    }
}
