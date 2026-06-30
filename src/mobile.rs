use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::{AvailabilityResponse, EnabledResponse, TorchRequest};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torchlight);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Torchlight<R>> {
    #[cfg(target_os = "android")]
    let handle = api
        .register_android_plugin("com.lembryo.tauri.plugin.torchlight", "TorchlightPlugin")
        .unwrap();
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torchlight).unwrap();
    Ok(Torchlight(handle))
}

/// Access to the torchlight APIs on mobile platforms.
pub struct Torchlight<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Torchlight<R> {
    /// Turn the torch on or off, optionally at a given brightness (`0.0..=1.0`).
    pub fn torch(&self, enabled: bool, level: Option<f64>) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("torch", TorchRequest { enabled, level })
            .map_err(Into::into)
    }

    /// Whether the device exposes a controllable torch.
    pub fn is_available(&self) -> crate::Result<bool> {
        self.0
            .run_mobile_plugin::<AvailabilityResponse>("isAvailable", ())
            .map(|response| response.available)
            .map_err(Into::into)
    }

    /// Whether the torch is currently lit.
    pub fn is_enabled(&self) -> crate::Result<bool> {
        self.0
            .run_mobile_plugin::<EnabledResponse>("isEnabled", ())
            .map(|response| response.enabled)
            .map_err(Into::into)
    }
}
