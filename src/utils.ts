export const ESC = "\x1B";
export const GS = "\x1D";

export const RESET = `${ESC}@`;
export const CUT_PARTIAL = `${GS}V\x01`;
export const CUT_FULL = `${GS}V\x00`;
export const FORM_FEED = "\f";

export const BOLD_ON = `${ESC}E\x01`;
export const BOLD_OFF = `${ESC}E\x00`;
export const UNDERLINE_ON = `${ESC}-\x01`;
export const UNDERLINE_OFF = `${ESC}-\x00`;
export const DOUBLE_HEIGHT_ON = `${GS}!\x01`;
export const DOUBLE_WIDTH_ON = `${GS}!\x10`;
export const DOUBLE_SIZE_ON = `${GS}!\x11`;
export const SIZE_NORMAL = `${GS}!\x00`;

export const ALIGN_LEFT = `${ESC}a\x00`;
export const ALIGN_CENTER = `${ESC}a\x01`;
export const ALIGN_RIGHT = `${ESC}a\x02`;

export function feedLines(n: number): string {
  return `${ESC}d${String.fromCharCode(Math.min(255, Math.max(0, n)))}`;
}

export const LINE_SPACING_DEFAULT = `${ESC}2`;

export function lineSpacing(n: number): string {
  return `${ESC}3${String.fromCharCode(Math.min(255, Math.max(0, n)))}`;
}

export const PAPER_WIDTH = {
  "58mm": 32,
  "76mm": 42,
  "80mm": 48,
  LX310: 80,
} as const;

export type PaperSize = keyof typeof PAPER_WIDTH;

export function divider(width: number = 48, char: string = "-"): string {
  return char.repeat(width);
}

export function centerText(text: string, width: number): string {
  if (text.length >= width) return text.slice(0, width);
  const left = Math.floor((width - text.length) / 2);
  return " ".repeat(left) + text;
}

export function rowLR(
  left: string,
  right: string,
  width: number = 48
): string {
  const space = width - left.length - right.length;
  return left + " ".repeat(Math.max(1, space)) + right;
}

export function row2Col(
  col1: string,
  col2: string,
  width: number = 48
): string {
  const w = Math.floor(width / 2);
  return col1.padEnd(w) + col2.padEnd(w);
}

export function wrapReceipt(content: string, feedCount: number = 3): string {
  return RESET + content + feedLines(feedCount) + CUT_PARTIAL;
}

export function wrapDotMatrix(content: string): string {
  return "\r\n" + content + "\r\n" + FORM_FEED;
}
