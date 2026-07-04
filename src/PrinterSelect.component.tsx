"use client";

import { useEffect, useState } from "react";
import { registry } from "@skalfa/skalfa-app-core";
import { listPrinters } from "./commands.js";
import type { PrinterInfo } from "./commands.js";

const STORAGE_KEY = "skalfa-printer-selected";

const SelectComponent = (props: any) => {
  const Comp = registry.get("SelectComponent");
  return Comp ? <Comp {...props} /> : null;
};

export interface PrinterSelectProps {
  name?: string;
  placeholder?: string;
  onChange?: (printerName: string) => void;
  value?: string;
}

export function PrinterSelect({
  name = "printer",
  placeholder = "-- PILIH PRINTER --",
  onChange,
  value: controlledValue,
}: PrinterSelectProps) {
  const [printers, setPrinters] = useState<PrinterInfo[]>([]);
  const [internalValue, setInternalValue] = useState<string>("");

  const isControlled = controlledValue !== undefined;
  const currentValue = isControlled ? controlledValue : internalValue;

  useEffect(() => {
    (async () => {
      try {
        const list = await listPrinters();
        setPrinters(list);

        if (!isControlled) {
          const saved = localStorage.getItem(STORAGE_KEY);
          if (saved && list.some((p) => p.name === saved)) {
            setInternalValue(saved);
          }
        }
      } catch {
        setPrinters([]);
      }
    })();
  }, [isControlled]);

  const handleChange = (val: string | string[]) => {
    const selected = Array.isArray(val) ? val[0] : val;
    if (!selected) return;

    localStorage.setItem(STORAGE_KEY, selected);

    if (!isControlled) {
      setInternalValue(selected);
    }

    onChange?.(selected);
  };

  const options = printers.map((p) => ({
    label: `${p.name}${p.is_default ? " ★" : ""} — ${p.status_text}`,
    value: p.name,
  }));

  return (
    <SelectComponent
      name={name}
      placeholder={placeholder}
      options={options}
      value={currentValue}
      onChange={handleChange}
    />
  );
}
