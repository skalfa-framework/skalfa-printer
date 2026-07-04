use crate::error::Error;

#[cfg(windows)]
use crate::printer_windows;

#[cfg(windows)]
use crate::printer_windows::PrinterInfo;

#[cfg(not(windows))]
use serde::Serialize;

#[cfg(not(windows))]
#[derive(Debug, Clone, Serialize)]
pub struct PrinterInfo {
    pub name: String,
    pub port: String,
    pub driver: String,
    pub is_default: bool,
    pub status: u32,
    pub status_text: String,
}

#[tauri::command]
pub fn list_printers() -> Result<Vec<PrinterInfo>, Error> {
    #[cfg(windows)]
    {
        printer_windows::enum_printers()
    }

    #[cfg(not(windows))]
    {
        Err(Error::UnsupportedPlatform)
    }
}

#[tauri::command]
pub fn print_raw(printer: String, content: String) -> Result<(), Error> {
    #[cfg(windows)]
    {
        printer_windows::print_raw(&printer, &content)
    }

    #[cfg(not(windows))]
    {
        let _ = (printer, content);
        Err(Error::UnsupportedPlatform)
    }
}

#[tauri::command]
pub fn get_printer_status(printer: String) -> Result<PrinterInfo, Error> {
    #[cfg(windows)]
    {
        printer_windows::get_printer_status(&printer)
    }

    #[cfg(not(windows))]
    {
        let _ = printer;
        Err(Error::UnsupportedPlatform)
    }
}
