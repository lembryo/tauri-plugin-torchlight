use tauri::{command, AppHandle, Runtime};

use crate::Result;
use crate::TorchlightExt;

#[command]
pub(crate) async fn torch_on<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.torchlight().torch_on()
}

#[command]
pub(crate) async fn torch_off<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.torchlight().torch_off()
}
