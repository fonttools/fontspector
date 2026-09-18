import { reactive, ref } from "vue";
import { CheckResult, Check, FontInfo, StatusCode, Profile } from "./types";

export const SORT_RESULT: Record<StatusCode, string> = {
  FATAL: "aa",
  FAIL: "bb",
  WARN: "cc",
  INFO: "dd",
  ERROR: "ee",
  PASS: "ff",
  SKIP: "zz",
};

export const state = reactive({
  version: "",
  allChecks: {} as Record<string, Check>,
  lastResults: null as CheckResult[] | null,
  currentFontName: "",
  loading: true,
  view: "start" as "start" | "classic" | "problem" | "listChecks",
  logLevel: "INFO",
  fullLists: false,
  selectedProfile: "universal" as Profile,
  activeCheckId: null as string | null,
  fonts: {} as Record<string, FontInfo>,
  error: null as string | null,
  counts: {
    FATAL: 0,
    FAIL: 0,
    WARN: 0,
    INFO: 0,
    ERROR: 0,
    PASS: 0,
    SKIP: 0,
  } as Record<StatusCode, number>,
});

export function updateResults(results: CheckResult[]) {
  state.lastResults = results;

  // Reset counts
  for (const status of Object.keys(state.counts) as StatusCode[]) {
    state.counts[status] = 0;
  }

  for (const result of results) {
    state.counts[result.worst_status]++;
  }
  if (results.length > 0) state.activeCheckId = results[0].check_id;
}

export function resetState() {
  state.view = "start";
  state.lastResults = null;
  state.fonts = {};
  for (const status of Object.keys(state.counts) as StatusCode[]) {
    state.counts[status] = 0;
  }
}
