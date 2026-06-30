use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
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

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(desktop)]
    #[test]
    fn unsupported_serializes_to_a_human_readable_string() {
        let json = serde_json::to_string(&Error::Unsupported).unwrap();
        assert_eq!(
            json,
            "\"torchlight is only supported on Android and iOS; this is a desktop platform\""
        );
    }
}
