import { useCallback, useEffect, useState } from "react";
import { listPrinters, printRaw, getPrinterStatus } from "./commands.js";
import type { PrinterInfo } from "./commands.js";

const STORAGE_KEY = "skalfa-printer-selected";

export interface UsePrinterReturn {
  printers: PrinterInfo[];
  selectedPrinter: string | null;
  selectPrinter: (name: string) => void;
  print: (content: string) => Promise<void>;
  printTo: (printer: string, content: string) => Promise<void>;
  status: PrinterInfo | null;
  isLoading: boolean;
  error: string | null;
  refresh: () => Promise<void>;
}

export function usePrinter(): UsePrinterReturn {
  const [printers, setPrinters] = useState<PrinterInfo[]>([]);
  const [selectedPrinter, setSelectedPrinter] = useState<string | null>(null);
  const [status, setStatus] = useState<PrinterInfo | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      setSelectedPrinter(saved);
    }
  }, []);

  const fetchPrinters = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const list = await listPrinters();
      setPrinters(list);

      if (!selectedPrinter) {
        const defaultPrinter = list.find((p) => p.is_default);
        if (defaultPrinter) {
          setSelectedPrinter(defaultPrinter.name);
          localStorage.setItem(STORAGE_KEY, defaultPrinter.name);
        }
      }
    } catch (e) {
      setError(typeof e === "string" ? e : String(e));
    } finally {
      setIsLoading(false);
    }
  }, [selectedPrinter]);

  useEffect(() => {
    fetchPrinters();
  }, []);

  useEffect(() => {
    if (!selectedPrinter) {
      setStatus(null);
      return;
    }

    let cancelled = false;

    getPrinterStatus(selectedPrinter)
      .then((info) => {
        if (!cancelled) setStatus(info);
      })
      .catch(() => {
        if (!cancelled) setStatus(null);
      });

    return () => {
      cancelled = true;
    };
  }, [selectedPrinter]);

  const selectPrinter = useCallback((name: string) => {
    setSelectedPrinter(name);
    localStorage.setItem(STORAGE_KEY, name);
  }, []);

  const print = useCallback(
    async (content: string) => {
      if (!selectedPrinter) {
        throw new Error("No printer selected. Call selectPrinter() first.");
      }
      setError(null);
      try {
        await printRaw(selectedPrinter, content);
      } catch (e) {
        const msg = typeof e === "string" ? e : String(e);
        setError(msg);
        throw new Error(msg);
      }
    },
    [selectedPrinter]
  );

  const printTo = useCallback(
    async (printer: string, content: string) => {
      setError(null);
      try {
        await printRaw(printer, content);
      } catch (e) {
        const msg = typeof e === "string" ? e : String(e);
        setError(msg);
        throw new Error(msg);
      }
    },
    []
  );

  const refresh = useCallback(async () => {
    await fetchPrinters();
    if (selectedPrinter) {
      try {
        const info = await getPrinterStatus(selectedPrinter);
        setStatus(info);
      } catch {
        setStatus(null);
      }
    }
  }, [fetchPrinters, selectedPrinter]);

  return {
    printers,
    selectedPrinter,
    selectPrinter,
    print,
    printTo,
    status,
    isLoading,
    error,
    refresh,
  };
}
