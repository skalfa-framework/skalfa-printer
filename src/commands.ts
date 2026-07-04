import { invoke } from "@tauri-apps/api/core";

export interface PrinterInfo {
  name: string;
  port: string;
  driver: string;
  is_default: boolean;
  status: number;
  status_text: string;
}

export async function listPrinters(): Promise<PrinterInfo[]> {
  return invoke<PrinterInfo[]>("plugin:skalfa-printer|list_printers");
}

export async function printRaw(
  printer: string,
  content: string
): Promise<void> {
  return invoke<void>("plugin:skalfa-printer|print_raw", { printer, content });
}

export async function getPrinterStatus(printer: string): Promise<PrinterInfo> {
  return invoke<PrinterInfo>("plugin:skalfa-printer|get_printer_status", {
    printer,
  });
}
