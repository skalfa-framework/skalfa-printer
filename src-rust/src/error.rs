use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to enumerate printers: {0}")]
    EnumPrinters(String),

    #[error("Failed to open printer '{0}': {1}")]
    OpenPrinter(String, String),

    #[error("Failed to start print job: {0}")]
    StartDoc(String),

    #[error("Failed to write to printer: {0}")]
    WritePrinter(String),

    #[error("Printer not found: {0}")]
    PrinterNotFound(String),

    #[error("Platform not supported")]
    UnsupportedPlatform,

    #[error("{0}")]
    Other(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
