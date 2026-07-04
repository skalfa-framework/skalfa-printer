const COMMANDS: &[&str] = &["list_printers", "print_raw", "get_printer_status"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
