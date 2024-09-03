use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torchlight);

pub fn init<R: Runtime, C: DeserializeOwned>(_app: &AppHandle<R>, api: PluginApi<R, C>) -> crate::Result<Torchlight<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.lembryo.tauri.plugin.torchlight", "TorchlightPlugin").unwrap();
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torchlight).unwrap();
    Ok(Torchlight(handle))
}

pub struct Torchlight<R: Runtime>(PluginHandle<R>);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorchRequest {
    enabled: bool,
}

impl<R: Runtime> Torchlight<R> {
    pub fn torch(&self, enabled: bool) -> crate::Result<()> {
        self
            .0
            .run_mobile_plugin("torch", TorchRequest {
                enabled,
            })
            .map_err(Into::into)
    }
}
