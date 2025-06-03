const fbWorker = new Worker(new URL("./webworker.js", import.meta.url));

import $ from "jquery";
import Dropzone from "dropzone";
import "bootstrap";
import { CheckSpecificRendering } from "./rendering";
import {
  FontInfo,
  StatusCode,
  CheckResult,
  Message,
  Status,
  Check,
  ReadyMessage,
} from "./types";
// @ts-ignore
let hbjs = window["hbjs"];

let fonts: Record<string, FontInfo> = {};
let allChecks: Record<string, Check> = {};

declare var CmarkGFM: any;
const tinysort = require("tinysort");

const SORT_RESULT: Record<StatusCode, string> = {
  FAIL: "aa",
  WARN: "bb",
  INFO: "cc",
  ERROR: "dd",
  PASS: "ee",
  SKIP: "zz",
};

const NOWASM = (s: string) => `
This check cannot be run in the web environment. This is because ${s}.
The web version of fontbakery is not a full replacement for the Python
version, and we recommend that you install fontbakery and check your
fonts locally to ensure that all checks are run.`;
const CANT_COMPILE = (s: string) =>
  NOWASM(`the ${s} library cannot be compiled for WASM`);
const NEEDS_NETWORK = NOWASM("it needs access to the network");
const BABELFONT = NOWASM(
  "the check requires a library (babelfont) with a Rust dependency"
);
const EXCUSES: Record<string, string> = {
  // Needs network
  "googlefonts/vendor_id": NEEDS_NETWORK,
  fontdata_namecheck: NEEDS_NETWORK,
  "googlefonts/vertical_metrics_regressions": NEEDS_NETWORK,
  "googlefonts/metadata/includes_production_subsets": NEEDS_NETWORK,
  "googlefonts/metadata/designer_profiles": NEEDS_NETWORK,
  "googlefonts/description/broken_links": NEEDS_NETWORK,
  "googlefonts/metadata/broken_links": NEEDS_NETWORK,
  "googlefonts/version_bump": NEEDS_NETWORK,
  "googlefonts/production_glyphs_similarity": NEEDS_NETWORK,
  // Shaping checks
  "googlefonts/render_own_name": CANT_COMPILE("Freetype"),
  dotted_circle: CANT_COMPILE("cffsubr [required by ufo2ft]"),
  // Other checks
  "googlefonts/metadata/family_directory_name": NOWASM(
    "there are no directories in the WASM environment"
  ),
};

/** Show that we have loaded the Python code, allow baking */
function showLoaded() {
  $("#loading").hide();
  $("#test").show();
  $("#listcheckscontainer").show();
}

/** Start again
 */
function reset() {
  $("#normalresults").show();
  $("#listchecks").hide();
  $("#startModal").show();
}

/** Display an error modal
 *
 * Used to display Python errors.
 * @param {string} msg - HTML error message
 */
function showError(msg: string) {
  $("#errorModal").show();
  $("#errorText").html(msg);
}

/** Record a result and add it to the output pills
 *
 * Used to display Python errors.
 * @param {Map} data - All the stuff
 */
function showResult(data: CheckResult[]) {
  console.log("Got a result", data);
  $("#startModal").hide();
  for (var result of data) {
    const tabid = $("#v-pills-tab").children().length;
    const checkid = result.check_id;
    let thispill = $(`#v-pills-tab button[data-checkid="${checkid}"]`);
    let worststatus = result.worst_status;
    $(`#${worststatus}-count`).html(
      (1 + parseInt($(`#${worststatus}-count`).html())).toString()
    );
    if (thispill.length == 0) {
      // Add a new pill
      thispill = $(`
          <button
            class="nav-link bg-${worststatus}"
            id="v-pills-${tabid}-tab"
            data-toggle="pill"
            data-target="#v-pills-${tabid}"
            data-sortorder="${SORT_RESULT[worststatus]}"
            type="button"
            role="tab"
            data-checkid=${checkid}
            aria-controls="v-pills-${tabid}">${result.check_name}</button>
        `);
      // Add a header if we need one
      $("#v-pills-tab").append(thispill);
      if (
        $(`#v-pills-tab button[data-sortorder=${SORT_RESULT[worststatus]}`)
          .length == 1
      ) {
        var header_sort = SORT_RESULT[worststatus].substring(0, 1);
        $("#v-pills-tab").append(
          $(`
          <button class="nav-link disabled header-${worststatus}" data-sortorder="${header_sort}">
          </div>
        `)
        );
      }
    }
    let thistab = $(`#v-pills-tabContent div[data-checkid="${checkid}"]`);
    if (thistab.length == 0) {
      thistab = $(`
        <div
          class="tab-pane fade"
          data-sortorder="${SORT_RESULT[worststatus]}"
          id="v-pills-${tabid}"
          role="tabpanel"
          aria-labelledby="v-pills-${tabid}-tab"
          data-checkid=${checkid}
        >
          <h4>${result.check_name}</h4>
          <p class="text-muted">${checkid}</p>
          <div class="rationale">
          ${CmarkGFM.convert(
            (result.check_rationale || "").replace(/^ +/gm, "")
          )}
          </div>
          <ul class="results">
          </ul>
        </div>
        `);
      $("#v-pills-tabContent").append(thistab);
    }
    // Update pill / tab results with worst result
    if (SORT_RESULT[worststatus] > thispill.data("sortorder")) {
      thispill.removeClass(function (index, className) {
        return (className.match(/(^|\s)bg-\S+/g) || []).join(" ");
      });
      thispill.addClass("bg-" + worststatus);
      thispill.data("sortorder", SORT_RESULT[worststatus]);
      thistab.data("sortorder", SORT_RESULT[worststatus]);
    }

    if (worststatus == "ERROR" && EXCUSES[checkid]) {
      thistab.find("ul.results").append(`<li>${EXCUSES[checkid]}</li>`);
    } else {
      const filename = result.filename || "Family Check";
      for (var log of result.subresults) {
        let where = "ul.results";
        where = `ul.results li ul[data-filename='${filename}']`;
        if (thistab.find(where).length == 0) {
          thistab.find("ul.results").append(`<li>
            <b>${filename}</b>
            <ul data-filename="${filename}">
            </ul>
          </li>`);
        }
        thistab.find(where).append(renderLog(log, checkid, result.filename));
      }
    }
  }
  // Sort the tabs based on result
  tinysort("div#v-pills-tab>button", { data: "sortorder" });
  tinysort("div#v-pills-tabContent>div", { data: "sortorder" });
  $("#v-pills-tab button").not(".disabled").first().tab("show");
}

function renderLog(log: Status, id: string, filename: string) {
  var extra_html = "";
  var suppress = false;
  if (log.metadata && id in CheckSpecificRendering) {
    [extra_html, suppress] = CheckSpecificRendering[id](
      log.metadata,
      fonts[filename]
    );
    if (suppress) {
      return $(extra_html);
    }
  }

  return $(`
    <li>
      <span
        class="bg-${log.severity} font-weight-bold">
        ${log.severity}
      </span>:
      <div>${CmarkGFM.convert(log.message || "")}</div>
      ${extra_html}
    </li>
  `);
}

/* Add a profile from the profiles list */
const PROFILES: Record<string, string> = {
  opentype: "OpenType (standards compliance)",
  universal: "Universal (community best practices)",
  googlefonts: "Google Fonts",
  iso15008: "ISO 15008 (in-car accessibility)",
  adobefonts: "Adobe Fonts",
  fontbureau: "Font Bureau",
  // typenetwork: "Type Network",
  fontwerk: "Fontwerk",
  microsoft: "Microsoft",
};

function addProfile(profilename: string, col: number) {
  const checked = profilename == "universal" ? "checked" : "";
  const widget = $(`
    <div class="form-check">
        <input class="form-check-input" type="radio" name="flexRadioDefault" id="profile-${profilename}" ${checked}>
        <label class="form-check-label" for="profile-${profilename}">
           ${PROFILES[profilename]}
        </label>
    </div>
  `);
  $(`#profiles .row #col${col}`).append(widget);
}

/**
 * Display all the checks
 *
 * @param {Map} checks: Metadata about the checks
 **/
function listChecks(checkName = "") {
  $("#startModal").hide();
  $("#listchecks").show();
  $("#normalresults").hide();
  $("#checks").empty();
  for (const id of Object.getOwnPropertyNames(allChecks)) {
    if (checkName && !id.includes(checkName)) {
      continue; // Skip this check
    }
    const check = allChecks[id];
    const card = $(`
      <div class="card my-4">
        <div class="card-header">
          <code>${id}</code>
        </div>
      <div class="card-body">
        <a name="${id}"><h2> ${check.description} </h2></a>
        ${CmarkGFM.convert(check.rationale || "")}
        <table class="table">
          <tr>
            <th>Sections</th>
            <td class="sections"></td>
          </tr>
          <tr>
            <th>Profiles</th>
            <td class="profiles"></td>
          </tr>
        </table>
      </div>
    </div>
    `);
    if (check.proposal) {
      if (check.proposal.length > 1) {
        let proposal_list = check.proposal.map(
          (p) => `<li><a href="${p}">${p}</a></li>`
        );
        card
          .find(".table")
          .prepend($(`<div>More information: <ul>${proposal_list}</ul></div>`));
      } else {
        card
          .find(".table")
          .prepend($(`<a href="${check.proposal[0]}">More information</a>`));
      }
    }
    for (const section of check.sections) {
      card
        .find(".sections")
        .append(
          $(
            `<span class="badge badge-pill badge-primary mr-2"> ${section} </span>`
          )
        );
    }
    for (const profile of check.profiles) {
      card
        .find(".profiles")
        .append(
          $(
            `<span class="badge badge-pill badge-primary mr-2"> ${profile} </span>`
          )
        );
    }
    $("#checks").append(card);
  }
}

fbWorker.onmessage = (event) => {
  let message = event.data as Message;
  console.log("Got a message", message);
  if ("ready" in message) {
    showLoaded();
    $("#fb-version").html(message.version);
    allChecks = message.checks;
    // If we have been asked to show a particular check, do that
    if (window.location.hash) {
      const checkName = window.location.hash.substring(1);
      if (checkName in allChecks) {
        listChecks(checkName);
      }
    }
    return;
  }
  if ("error" in message) {
    showError(message.error);
  } else {
    $("#v-pills-tab button:first-child").tab("show");
    showResult(message as CheckResult[]);
  }
};

Dropzone.autoDiscover = false;

$(function () {
  Dropzone.options.dropzone = {
    url: "https://127.0.0.1/", // This doesn't matter
    maxFilesize: 10, // Mb
    accept: function (file, done) {
      const reader = new FileReader();
      reader.addEventListener("loadend", function (event) {
        var filedata = new Uint8Array(event.target.result as ArrayBuffer);
        var blob = hbjs.createBlob(filedata);
        var face = hbjs.createFace(blob, 0);
        var font = hbjs.createFont(face);
        fonts[file.name] = {
          name: file.name,
          file: filedata,
          blob,
          face,
          font,
        };
      });
      reader.readAsArrayBuffer(file);
    },
  };
  Dropzone.discover();
  $('[data-toggle="tooltip"]').tooltip();
  Object.keys(PROFILES).forEach((profilename, ix) => {
    addProfile(profilename, ix % 2);
  });
  $("#startModal").show();
  $("#test").click(function () {
    const profile = $("#profiles .form-check-input:checked")[0].id.replace(
      "profile-",
      ""
    );
    const fulllists = $("#full-lists").is(":checked");
    const loglevels = $("#loglevels").val();
    var files: Record<string, Uint8Array> = {};
    for (var filename of Object.keys(fonts)) {
      files[filename] = fonts[filename].file;
    }
    console.log(files);
    fbWorker.postMessage({ profile, files, loglevels, fulllists });
  });
  $("#listchecksbtn").on("click", () => listChecks());
  $(".leftarrow").click(reset);
  fbWorker.postMessage({ id: "justload" });
});
