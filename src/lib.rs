use tauri::{plugin::{Builder, TauriPlugin}, Manager, Runtime};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

use crate::commands::torch;

#[cfg(desktop)]
use desktop::Torchlight;
#[cfg(mobile)]
use mobile::Torchlight;

pub trait TorchlightExt<R: Runtime> {
    fn torchlight(&self) -> &Torchlight<R>;
}

impl<R: Runtime, T: Manager<R>> TorchlightExt<R> for T {
    fn torchlight(&self) -> &Torchlight<R> {
        self.state::<Torchlight<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("torchlight")
        .invoke_handler(tauri::generate_handler![
            torch,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let torchlight = mobile::init(app, api).unwrap();
            #[cfg(desktop)]
            let torchlight = desktop::init(app, api).unwrap();
            app.manage(torchlight);
            Ok(())
        })
        .build()
}
