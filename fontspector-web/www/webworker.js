var module = import("../pkg/fontspector_web.js");

async function init() {
  console.log("Loading the module");
  let wasm = await module;
  console.log("Loaded");
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

  try {
    const version = wasm.version();
    const checks = wasm.dump_checks();
    self.postMessage({
      ready: true,
      version: version,
      checks: JSON.parse(checks),
    });
  } catch (error) {
    self.postMessage({ error });
    return;
  }

  self.onmessage = async (event) => {
    // make sure loading is done
    const { id, files, profile, loglevels, fulllists } = event.data;
    self.profile = profile;

    if (id == "justload") {
      return;
    }

    const callback = (msg) => self.postMessage(msg.toJs());

    self.loglevels = loglevels;
    self.fulllists = fulllists;
    self.exclude_checks = EXCLUDE_CHECKS;
    try {
      const results = JSON.parse(wasm.check_fonts(files, profile));
      self.postMessage(results);
    } catch (error) {
      self.postMessage({ error: error, id });
    }
  };
}
init();
