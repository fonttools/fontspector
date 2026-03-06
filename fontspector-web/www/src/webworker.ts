import {
  version,
  dump_checks,
  fix_fonts,
  best_family_name,
  check_fonts,
} from "../../pkg/fontspector_web.js";
import {
  CheckResult,
  ErrorReply,
  FixRequest,
  NameReply,
  ReadyReply,
  ReplyMessage,
  RequestMessage,
} from "./types";

let loadedFonts: Record<string, Uint8Array> = {};

function post(message: ReplyMessage) {
  try {
    self.postMessage(message);
  } catch (error: any) {
    self.postMessage({ error: error.toString() } as ErrorReply);
  }
}

async function init() {
  const EXCLUDE_CHECKS = [
    "fontbakery_version", // We download the latest each time
    "ufo_required_fields",
    "ufo_recommended_fields",
    "designspace_has_sources",
    "designspace_has_default_master",
    "designspace_has_consistent_glyphset",
    "designspace_has_consistent_codepoints",
    "shaping/regression",
    "shaping/forbidden",
    "shaping/collides",
    "fontv", // Requires a subprocess
  ];

  post({
    id: "ready",
    ready: true,
    version: version(),
    checks: JSON.parse(dump_checks()),
  });

  self.onmessage = async (event) => {
    // make sure loading is done
    let msg: RequestMessage = event.data;

    if (msg.id == "justload") {
      return;
    } else if (msg.id == "fix") {
      const { requests } = msg as FixRequest;
      try {
        const zipfile = fix_fonts(loadedFonts, requests);
        post({ id: "fix_result", zipfile });
      } catch (error: any) {
        post({ id: "error", error: error.toString() });
      }
    } else if (msg.id == "run_checks") {
      const name = best_family_name(msg.files);
      post({ id: "name", name });
      loadedFonts = msg.files;
      const results: CheckResult[] = JSON.parse(
        check_fonts(msg.files, msg.profile, msg.fulllists, msg.loglevels),
      );
      post({ id: "check_result", results });
    } else {
      post({ id: "error", error: "Unknown message:" + JSON.stringify(msg) });
    }
  };
}
init();
