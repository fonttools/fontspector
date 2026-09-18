import { PROFILES } from "./constants";

export type FontInfo = {
  name: string;
  file: Uint8Array;
  blob: any;
  face: any;
  font: any;
};

export type StatusCode =
  | "FAIL"
  | "FATAL"
  | "WARN"
  | "INFO"
  | "ERROR"
  | "PASS"
  | "SKIP";

export interface GlyphProblem {
  /** The name of the glyph */
  glyph_name: string;
  /** The ID of the glyph */
  glyph_id: number;
  /** A specific location within the font's design space, in user-space coordinates. */
  userspace_location?: Record<string, number> | null;
  /** A specific location within the glyph's coordinate space. */
  position?: [number, number] | null;
  /** The value that was found. */
  actual?: any | null;
  /** The value that was expected. */
  expected?: any | null;
  /** A description of the problem to show to the user. */
  message: string;
}

/** A problem with a specific OpenType table. */
export interface TableProblem {
  /** The tag of the table */
  table_tag: string;
  /** The field within the table which has the problem, if any. */
  field_name: string | null;
  /** The value of the field which has the problem, if any. */
  actual: any | null;
  /** The expected value of the field, if any. */
  expected: any | null;
  /** A description of the problem to show to the user. */
  message: string;
}

/** A problem which is not specific to a glyph or table. */
export interface FontProblem {
  /** A description of the problem to show to the user. */
  message: string;
  /** Additional context about the problem */
  context: any | null;
}

export interface Choice {
  /** The value returned when this choice is selected. */
  value: string;
  /** The human-readable text shown to the user. */
  description: string;
}

export type ChoiceDialogFieldType = {
  Choice: Choice[];
};

export type DialogFieldType =
  | ChoiceDialogFieldType
  | "Text"
  | "Number"
  | "Boolean";

export interface DialogField {
  /** Unique key for this field in the replies map. */
  key: string;
  /** Prompt shown to the user. */
  prompt: string;
  /** Control type for this field. */
  field_type: DialogFieldType;
}

/**
 * Newtype wrapper in Rust serialized as a plain array of dialog fields.
 */
export type MoreInfoRequest = DialogField[];

export type MoreInfoReplyValue = string | number | boolean | null;

/**
 * Reply payload keyed by dialog field key.
 */
export type MoreInfoReply = Record<string, MoreInfoReplyValue>;

export type FixNeedsMoreInformation = {
  FixNeedsMoreInformation: MoreInfoRequest;
};

export type Metadata =
  | { GlyphProblem: GlyphProblem }
  | { TableProblem: TableProblem }
  | { FontProblem: FontProblem }
  | FixNeedsMoreInformation
  | { Other: any };

export function isGlyphProblem(
  metadata: Metadata,
): metadata is { GlyphProblem: GlyphProblem } {
  return "GlyphProblem" in metadata;
}

export function isTableProblem(
  metadata: Metadata,
): metadata is { TableProblem: TableProblem } {
  return "TableProblem" in metadata;
}

export function isFontProblem(
  metadata: Metadata,
): metadata is { FontProblem: FontProblem } {
  return "FontProblem" in metadata;
}

export function isFixNeedsMoreInformation(
  metadata: Metadata,
): metadata is FixNeedsMoreInformation {
  return "FixNeedsMoreInformation" in metadata;
}

export function isChoiceDialogFieldType(
  fieldType: DialogFieldType,
): fieldType is ChoiceDialogFieldType {
  return (
    typeof fieldType === "object" && fieldType !== null && "Choice" in fieldType
  );
}

export type Check = {
  description: string;
  rationale: string;
  proposal: string[];
  sections: string[];
  profiles: string[];
};

export type Status = {
  message?: string | null;
  severity: StatusCode;
  code?: string | null;
  metadata?: Metadata[] | null;
};
export type CheckResult = {
  check_id: string;
  check_name: string;
  check_rationale: string;
  filename: string | null;
  section: string | null;
  subresults: Status[];
  worst_status: StatusCode;
  hotfix_available: boolean;
  sourcefix_available: boolean;
};

export interface SubresultWithCheck {
  status: Status;
  check: CheckResult;
}

// Types related to fixing fonts
export interface FixItem {
  filename: string;
  check_id: string;
  details?: MoreInfoReply | null;
}

// Messages from the front-end to the web worker, and from the web worker to the front-end
export interface ErrorReply {
  id: "error";
  error: string;
}
export interface NameReply {
  id: "name";
  name: string;
}
export interface ReadyReply {
  id: "ready";
  ready: boolean;
  version: string;
  checks: Record<string, Check>;
}

export interface FixReply {
  id: "fix_result";
  zipfile: Uint8Array;
  download?: boolean;
}

export type ReplyMessage =
  | ErrorReply
  | NameReply
  | ReadyReply
  | FixReply
  | { id: "check_result"; results: CheckResult[] };

export type Profile = keyof typeof PROFILES;

export interface FixRequest {
  id: "fix";
  requests: FixItem[];
  download?: boolean;
}

export interface RunCheckRequest {
  id: "run_checks";
  profile: Profile;
  files: Record<string, Uint8Array>;
  loglevels: string;
  fulllists: boolean;
}
export type LoadFontspectorRequest = { id: "justload" };
export type RequestMessage =
  | FixRequest
  | RunCheckRequest
  | LoadFontspectorRequest;
