import { StatusCode } from "./types";

// Maps every StatusCode to its display metadata.
const STATUS_MAP: Record<
  StatusCode,
  { label: string; emoji: string; color: string }
> = {
  FATAL: { label: "Fatal", emoji: "💀", color: "dark" },
  ERROR: { label: "Error", emoji: "💥", color: "danger" },
  FAIL: { label: "Fail", emoji: "🔥", color: "danger" },
  WARN: { label: "Warn", emoji: "⚠️", color: "warning" },
  INFO: { label: "Info", emoji: "ℹ️", color: "info" },
  PASS: { label: "Pass", emoji: "✅", color: "success" },
  SKIP: { label: "Skip", emoji: "⏩", color: "secondary" },
};

// Ordered list of status codes for iteration (e.g. rendering in severity order).
export const STATUS_ORDER: StatusCode[] = [
  "FATAL",
  "ERROR",
  "FAIL",
  "WARN",
  "INFO",
  "PASS",
  "SKIP",
];

export function worseThan(a: StatusCode, b: StatusCode): boolean {
  return STATUS_ORDER.indexOf(a) <= STATUS_ORDER.indexOf(b);
}

export const STATUS_LABELS: Record<StatusCode, string> = Object.fromEntries(
  STATUS_ORDER.map((c) => [c, STATUS_MAP[c].label]),
) as Record<StatusCode, string>;

export const EMOJIS: Record<StatusCode, string> = Object.fromEntries(
  STATUS_ORDER.map((c) => [c, STATUS_MAP[c].emoji]),
) as Record<StatusCode, string>;

export const SEVERITY_COLOR: Record<StatusCode, string> = Object.fromEntries(
  STATUS_ORDER.map((c) => [c, STATUS_MAP[c].color]),
) as Record<StatusCode, string>;

export const PROFILES = {
  opentype: "OpenType (standards compliance)",
  universal: "Universal (community best practices)",
  googlefonts: "Google Fonts",
  iso15008: "ISO 15008 (in-car accessibility)",
  adobefonts: "Adobe Fonts",
  fontbureau: "Font Bureau",
  fontwerk: "Fontwerk",
  microsoft: "Microsoft",
  workspace: "Google Workspace",
  typenetwork: "Type Network",
};
