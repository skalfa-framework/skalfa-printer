use crate::error::Error;
use serde::Serialize;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HANDLE};
use windows::Win32::Graphics::Printing::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, EnumPrintersW, GetDefaultPrinterW,
    OpenPrinterW, StartDocPrinterW, StartPagePrinter, WritePrinter, DOC_INFO_1W,
    PRINTER_DEFAULTSW, PRINTER_ENUM_LOCAL, PRINTER_ENUM_CONNECTIONS,
    PRINTER_INFO_2W,
};

#[derive(Debug, Clone, Serialize)]
pub struct PrinterInfo {
    pub name: String,
    pub port: String,
    pub driver: String,
    pub is_default: bool,
    pub status: u32,
    pub status_text: String,
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn pwstr_to_string(ptr: *const u16) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }

    let slice = std::slice::from_raw_parts(ptr, len);
    String::from_utf16_lossy(slice)
}

pub fn enum_printers() -> Result<Vec<PrinterInfo>, Error> {
    let default_printer = get_default_printer_name().unwrap_or_default();

    let flags = PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS;
    let mut bytes_needed: u32 = 0;
    let mut count: u32 = 0;

    unsafe {
        let _ = EnumPrintersW(
            flags,
            PCWSTR::null(),
            2,
            None,
            &mut bytes_needed,
            &mut count,
        );
    }

    if bytes_needed == 0 {
        return Ok(Vec::new());
    }

    let mut buffer: Vec<u8> = vec![0u8; bytes_needed as usize];

    unsafe {
        EnumPrintersW(
            flags,
            PCWSTR::null(),
            2,
            Some(&mut buffer),
            &mut bytes_needed,
            &mut count,
        )
        .map_err(|e| Error::EnumPrinters(e.to_string()))?;
    }

    let printers_ptr = buffer.as_ptr() as *const PRINTER_INFO_2W;
    let mut result = Vec::with_capacity(count as usize);

    for i in 0..count as isize {
        unsafe {
            let info = &*printers_ptr.offset(i);
            let name = pwstr_to_string(info.pPrinterName.0);
            let port = pwstr_to_string(info.pPortName.0);
            let driver = pwstr_to_string(info.pDriverName.0);
            let status = info.Status;

            result.push(PrinterInfo {
                is_default: name == default_printer,
                name,
                port,
                driver,
                status,
                status_text: status_to_string(status),
            });
        }
    }

    Ok(result)
}

pub fn print_raw(printer_name: &str, content: &str) -> Result<(), Error> {
    let wide_name = to_wide(printer_name);

    let mut handle = HANDLE::default();

    let defaults = PRINTER_DEFAULTSW::default();

    unsafe {
        OpenPrinterW(
            PCWSTR(wide_name.as_ptr()),
            &mut handle,
            Some(&defaults),
        )
        .map_err(|e| Error::OpenPrinter(printer_name.to_string(), e.to_string()))?;
    }

    let result = send_raw_document(handle, printer_name, content);

    unsafe {
        let _ = ClosePrinter(handle);
    }

    result
}

pub fn get_printer_status(printer_name: &str) -> Result<PrinterInfo, Error> {
    let printers = enum_printers()?;
    printers
        .into_iter()
        .find(|p| p.name == printer_name)
        .ok_or_else(|| Error::PrinterNotFound(printer_name.to_string()))
}

fn send_raw_document(handle: HANDLE, printer_name: &str, content: &str) -> Result<(), Error> {
    let doc_name = to_wide("Skalfa Print Job");
    let datatype = to_wide("RAW");

    let doc_info = DOC_INFO_1W {
        pDocName: windows::core::PWSTR(doc_name.as_ptr() as *mut _),
        pOutputFile: windows::core::PWSTR::null(),
        pDatatype: windows::core::PWSTR(datatype.as_ptr() as *mut _),
    };

    unsafe {
        let job_id = StartDocPrinterW(handle, 1, &doc_info as *const _ as *const _);
        if job_id == 0 {
            return Err(Error::StartDoc(format!(
                "StartDocPrinterW returned 0 for printer '{}'",
                printer_name
            )));
        }

        let start_page_result: BOOL = StartPagePrinter(handle);
        if !start_page_result.as_bool() {
            let _ = EndDocPrinter(handle);
            return Err(Error::WritePrinter("StartPagePrinter failed".to_string()));
        }

        let data = content.as_bytes();
        let mut bytes_written: u32 = 0;

        let write_result: BOOL = WritePrinter(
            handle,
            data.as_ptr() as *const _,
            data.len() as u32,
            &mut bytes_written,
        );

        if !write_result.as_bool() {
            let _ = EndPagePrinter(handle);
            let _ = EndDocPrinter(handle);
            return Err(Error::WritePrinter("WritePrinter failed".to_string()));
        }

        let _ = EndPagePrinter(handle);
        let _ = EndDocPrinter(handle);
    }

    Ok(())
}

fn get_default_printer_name() -> Option<String> {
    let mut size: u32 = 0;

    unsafe {
        let _ = GetDefaultPrinterW(windows::core::PWSTR::null(), &mut size);
    }

    if size == 0 {
        return None;
    }

    let mut buffer: Vec<u16> = vec![0u16; size as usize];

    unsafe {
        let result: BOOL = GetDefaultPrinterW(
            windows::core::PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );
        if !result.as_bool() {
            return None;
        }
    }

    if let Some(pos) = buffer.iter().position(|&c| c == 0) {
        buffer.truncate(pos);
    }

    Some(String::from_utf16_lossy(&buffer))
}

fn status_to_string(status: u32) -> String {
    if status == 0 {
        return "Ready".to_string();
    }

    let mut parts = Vec::new();

    if status & 0x00000001 != 0 {
        parts.push("Paused");
    }
    if status & 0x00000002 != 0 {
        parts.push("Error");
    }
    if status & 0x00000004 != 0 {
        parts.push("Pending Deletion");
    }
    if status & 0x00000008 != 0 {
        parts.push("Paper Jam");
    }
    if status & 0x00000010 != 0 {
        parts.push("Paper Out");
    }
    if status & 0x00000020 != 0 {
        parts.push("Manual Feed");
    }
    if status & 0x00000040 != 0 {
        parts.push("Paper Problem");
    }
    if status & 0x00000080 != 0 {
        parts.push("Offline");
    }
    if status & 0x00000100 != 0 {
        parts.push("IO Active");
    }
    if status & 0x00000200 != 0 {
        parts.push("Busy");
    }
    if status & 0x00000400 != 0 {
        parts.push("Printing");
    }
    if status & 0x00000800 != 0 {
        parts.push("Output Bin Full");
    }
    if status & 0x00001000 != 0 {
        parts.push("Not Available");
    }
    if status & 0x00002000 != 0 {
        parts.push("Waiting");
    }
    if status & 0x00004000 != 0 {
        parts.push("Processing");
    }
    if status & 0x00008000 != 0 {
        parts.push("Initializing");
    }
    if status & 0x00010000 != 0 {
        parts.push("Warming Up");
    }
    if status & 0x00020000 != 0 {
        parts.push("Toner Low");
    }
    if status & 0x00040000 != 0 {
        parts.push("No Toner");
    }
    if status & 0x00080000 != 0 {
        parts.push("Page Punt");
    }
    if status & 0x00400000 != 0 {
        parts.push("Door Open");
    }
    if status & 0x01000000 != 0 {
        parts.push("Power Save");
    }

    if parts.is_empty() {
        format!("Unknown (0x{:08X})", status)
    } else {
        parts.join(", ")
    }
}
