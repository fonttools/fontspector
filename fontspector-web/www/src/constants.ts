import { StatusCode } from "./types";

export const STATUS_LABELS: Record<StatusCode, string> = {
  ERROR: "Error",
  FAIL: "Fail",
  WARN: "Warn",
  SKIP: "Skip",
  INFO: "Info",
  PASS: "Pass",
  FATAL: "Fatal",
};

export const EMOJIS: Record<StatusCode, string> = {
  ERROR: "💥",
  FAIL: "🔥",
  WARN: "⚠️",
  SKIP: "⏩",
  INFO: "ℹ️",
  PASS: "✅",
  FATAL: "💀",
};

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
};

export const SEVERITY_COLOR: Record<StatusCode, string> = {
  ERROR: "danger",
  FAIL: "danger",
  WARN: "warning",
  SKIP: "secondary",
  INFO: "info",
  PASS: "success",
  FATAL: "dark",
};
