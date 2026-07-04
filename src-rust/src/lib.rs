mod commands;
mod error;

#[cfg(windows)]
mod printer_windows;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("skalfa-printer")
        .invoke_handler(tauri::generate_handler![
            commands::list_printers,
            commands::print_raw,
            commands::get_printer_status,
        ])
        .build()
}
