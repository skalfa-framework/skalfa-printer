import { registry } from "@skalfa/skalfa-app-core";
import { PrinterSelect } from "./PrinterSelect.component.js";

export * from "./commands.js";
export * from "./usePrinter.hook.js";
export * from "./PrinterSelect.component.js";
export * from "./utils.js";

registry.register("PrinterSelect", PrinterSelect);
