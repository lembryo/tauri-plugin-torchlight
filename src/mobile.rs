use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::{
    AvailabilityResponse, EnabledResponse, TorchCapabilities, TorchRequest, ToggleRequest,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torchlight);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Torchlight<R>> {
    // Registration genuinely fails when the native class cannot be loaded
    // (stripped by R8, a stale AAR, a package rename, a JNI error), so it is
    // propagated instead of panicking the app during setup.
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.lembryo.tauri.plugin.torchlight", "TorchlightPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torchlight)?;
    Ok(Torchlight(handle))
}

/// Access to the torchlight APIs on mobile platforms.
pub struct Torchlight<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Torchlight<R> {
    /// Turn the torch on or off, optionally at a given brightness.
    pub fn torch(&self, enabled: bool, level: Option<f64>) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("torch", TorchRequest { enabled, level })
            .map_err(Into::into)
    }

    /// Flip the torch natively and report the resulting state.
    pub fn toggle(&self, level: Option<f64>) -> crate::Result<bool> {
        self.0
            .run_mobile_plugin::<EnabledResponse>("toggle", ToggleRequest { level })
            .map(|response| response.enabled)
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

    /// What this device's torch can actually do.
    pub fn capabilities(&self) -> crate::Result<TorchCapabilities> {
        self.0
            .run_mobile_plugin::<TorchCapabilities>("capabilities", ())
            .map_err(Into::into)
    }
}
