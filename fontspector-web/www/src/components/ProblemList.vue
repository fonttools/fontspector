<script setup lang="ts">
import { ref } from 'vue';
import { renderMarkdown } from '../markdown';
import { SEVERITY_COLOR } from '../constants';
import { FixItem, FixRequest, isFontProblem, isGlyphProblem, isTableProblem, StatusCode, SubresultWithCheck } from '../types';
import fbWorker from '../workersingleton';
const props = defineProps<{
  resultClass: string,
  fixable: boolean,
  results: any[],
}>();

function checkArea(sr: SubresultWithCheck): string {
  // Look at the metadata for this specific status, not all subresults
  const metadata = sr.status.metadata || [];
  if (metadata.length === 0) return sr.check.section || sr.check.check_name;

  // Use the first metadata item for this status
  const meta = metadata[0];
  if (isGlyphProblem(meta)) return `Glyph ${meta.GlyphProblem.glyph_name}`;
  if (isTableProblem(meta)) return `Table ${meta.TableProblem.table_tag}`;
  if (isFontProblem(meta)) return "Font";
  return "Unknown";
}


function groupByArea(results: SubresultWithCheck[]): Record<string, SubresultWithCheck[]> {
  const groups: Record<string, SubresultWithCheck[]> = {};
  for (const res of results) {
    const area = checkArea(res);
    if (!groups[area]) groups[area] = [];
    groups[area].push(res);
  }
  return groups;
}

function makeFixKey(checkId: string, filename: string): string {
  return `${checkId}::${filename}`;
}

function selectAllChildren(event: Event) {
  const checkbox = event.target as HTMLInputElement;
  const details = checkbox.closest("details");

  // If not in a details element, we're at the top level - select all checkboxes in the entire component
  const container = details || checkbox.closest("div");
  if (!container) return;

  const childCheckboxes = container.querySelectorAll("input[type=checkbox].individual-fix");
  childCheckboxes.forEach(cb => {
    const checkId = cb.getAttribute("data-checkid");
    const filenames = JSON.parse(cb.getAttribute("data-filenames") || "[]");
    if (checkId) {
      if (checkbox.checked) {
        filenames.forEach((filename: string) => {
          const key = makeFixKey(checkId, filename);
          selectedFixRequests.value.set(key, {
            check_id: checkId,
            filename,
            details: null,
          });
        });
      } else {
        filenames.forEach((filename: string) => {
          const key = makeFixKey(checkId, filename);
          selectedFixRequests.value.delete(key);
        });
      }
      (cb as HTMLInputElement).checked = checkbox.checked;
    }
  });
}
function fileCount(results: SubresultWithCheck[]): number {
  const files = new Set<string>();
  for (const res of results) {
    files.add(res.check.filename || "Family Check");
  }
  return files.size;
}

type MessageVariant = {
  signature: string;
  severity: StatusCode;
  code: string | null;
  message: string | null;
};

type CollatedCheckGroup = {
  check: SubresultWithCheck['check'];
  filenames: string[];
  messages: MessageVariant[];
  worstSeverity: StatusCode;
};

const SEVERITY_ORDER: Record<StatusCode, number> = {
  FATAL: 7,
  ERROR: 6,
  FAIL: 5,
  WARN: 4,
  INFO: 3,
  PASS: 2,
  SKIP: 1,
};

function resultFilename(res: SubresultWithCheck): string {
  return res.check.filename || 'Family Check';
}

function statusSignature(res: SubresultWithCheck): string {
  return JSON.stringify({
    severity: res.status.severity,
    code: res.status.code || null,
    message: res.status.message || null,
  });
}

// Cluster by check first, then by file cohorts that share the same set of messages.
function collateFiles(results: SubresultWithCheck[]): CollatedCheckGroup[] {
  const checkGroups = new Map<string, SubresultWithCheck[]>();
  for (const res of results) {
    const key = res.check.check_id;
    if (!checkGroups.has(key)) checkGroups.set(key, []);
    checkGroups.get(key)!.push(res);
  }

  const collated: CollatedCheckGroup[] = [];
  for (const entries of checkGroups.values()) {
    const byFile = new Map<string, SubresultWithCheck[]>();
    const messageBySignature = new Map<string, MessageVariant>();

    for (const entry of entries) {
      const filename = resultFilename(entry);
      if (!byFile.has(filename)) byFile.set(filename, []);
      byFile.get(filename)!.push(entry);

      const signature = statusSignature(entry);
      if (!messageBySignature.has(signature)) {
        messageBySignature.set(signature, {
          signature,
          severity: entry.status.severity,
          code: entry.status.code || null,
          message: entry.status.message || null,
        });
      }
    }

    const cohorts = new Map<string, { filenames: string[]; signatures: string[] }>();
    for (const [filename, fileEntries] of byFile.entries()) {
      const signatures = Array.from(new Set(fileEntries.map((entry) => statusSignature(entry)))).sort();
      const cohortKey = signatures.join('|::|');
      if (!cohorts.has(cohortKey)) {
        cohorts.set(cohortKey, { filenames: [], signatures });
      }
      cohorts.get(cohortKey)!.filenames.push(filename);
    }

    for (const cohort of cohorts.values()) {
      const messages = cohort.signatures
        .map((signature) => messageBySignature.get(signature))
        .filter((msg): msg is MessageVariant => msg !== undefined);

      const worstSeverity = messages.reduce<StatusCode>((worst, msg) => {
        return SEVERITY_ORDER[msg.severity] > SEVERITY_ORDER[worst] ? msg.severity : worst;
      }, messages[0]?.severity || entries[0].status.severity);

      collated.push({
        check: entries[0].check,
        filenames: cohort.filenames.sort((a, b) => a.localeCompare(b)),
        messages,
        worstSeverity,
      });
    }
  }

  return collated.sort((a, b) => {
    const byName = a.check.check_name.localeCompare(b.check.check_name);
    if (byName !== 0) return byName;
    const byCount = b.filenames.length - a.filenames.length;
    if (byCount !== 0) return byCount;
    return a.filenames[0].localeCompare(b.filenames[0]);
  });
}
function fix(download: boolean) {
  // Create a FixRequestPackage
  try {
    let items: FixItem[] = [];
    selectedFixRequests.value.forEach(item => items.push({
      // These are proxies, deproxify them
      check_id: item.check_id,
      filename: item.filename,
    }));
    const fixRequestPackage: FixRequest = {
      id: "fix",
      requests: items,
      download,
    };
    // Send it to the webworker
    console.log("Sending fix request package to worker:", fixRequestPackage);
    fbWorker.postMessage(fixRequestPackage);
  } catch (e) {
    console.error("Error preparing fix request package:", e);
  }
}

const selectedFixRequests = ref<Map<string, FixItem>>(new Map());

function toggleFixRequest(checkId: string, filenames: string[], e: Event) {
  for (var filename of filenames) {
    const key = makeFixKey(checkId, filename);
    if ((e.target as HTMLInputElement).checked) {
      selectedFixRequests.value.set(key, {
        check_id: checkId,
        filename,
        details: null,
      });
    } else {
      selectedFixRequests.value.delete(key);
    }
  }
}

function baseName(path: string): string {
  return path.split('/').slice(-1)[0];
}

</script>
<template>
  <div>
    <p><span class="resultclass">{{ resultClass }}</span> ({{ results.length }} issues across {{
      fileCount(results) }}
      files)
      <span v-if="fixable" class="float-end">
        Fix all: <input type="checkbox" @change="selectAllChildren" /></span>
    </p>
    <details v-for="([area, group], idx) of Object.entries(groupByArea(results)).sort(([a], [b]) => a.localeCompare(b))"
      :key="idx" open="true">
      <summary class="mb-2">{{ area }} <span v-if="fixable && collateFiles(group).length > 1" class="float-end">Fix {{
        group.length
          }} issues:
          <input type="checkbox" @change="selectAllChildren" /></span>
      </summary>

      <details v-for="(checkGroup, idx) in collateFiles(group)" :key="idx" class="mb-3">
        <summary>
          <span :class="`badge status-badge text-bg-${SEVERITY_COLOR[checkGroup.worstSeverity]} m-1`">{{
            checkGroup.worstSeverity
          }}</span>
          <span class="checkname">{{ checkGroup.check.check_name }}</span>
          <span v-if="checkGroup.filenames.length > 1" class="affected-files"> (Affects {{ checkGroup.filenames.length
            }} files)</span><span v-if="checkGroup.filenames.length == 1" class="affected-files"> (Affects {{
              baseName(checkGroup.filenames[0]) }})</span>
          <span v-if="fixable" style="float: right;">
            Fix:
            <input type="checkbox" class="individual-fix" :data-checkid="checkGroup.check.check_id"
              :data-filenames="JSON.stringify(checkGroup.filenames)"
              :checked="checkGroup.filenames.every((filename) => selectedFixRequests.has(makeFixKey(checkGroup.check.check_id, filename)))"
              @change="(e) => { toggleFixRequest(checkGroup.check.check_id, checkGroup.filenames, e) }" />
          </span>
        </summary>
        <div class="affected-files" v-if="checkGroup.filenames.length > 1">Affects
          <ul>
            <li v-for='file in checkGroup.filenames' :key='file'>
              {{ file }}
            </li>
          </ul>
        </div>
        <p class="rationale" v-html="renderMarkdown(checkGroup.check.check_rationale)"></p>
        <div v-for="message in checkGroup.messages" :key="message.signature" class="message-block">
          <span :class="`badge text-bg-${SEVERITY_COLOR[message.severity]} m-1`">{{ message.severity }}</span>
          <span v-if="message.code" class="message-code">{{ message.code }}</span>
          <p class="message" v-html="renderMarkdown(message.message || '')"></p>
        </div>
      </details>
    </details>
    <div class="clearfix">
      <div class="float-end" v-if="fixable">
        <button class="btn btn-primary" @click="() => fix(false)" :disabled="selectedFixRequests.size == 0">Fix
          {{ selectedFixRequests.size }} Selected Issues</button>
        <button class="btn btn-primary" @click="() => fix(true)" :disabled="selectedFixRequests.size == 0">Fix and
          Download</button>
      </div>
    </div>
  </div>
</template>

<style>
.resultclass {
  font-weight: bold;
  font-size: 1.2em;
}

details>details {
  margin-left: 20px;
}

details+details {
  margin-top: 10px;
}

.rationale {
  background-color: color-mix(in srgb, var(--bs-body-bg), var(--opposite) 5%);

  padding: 10px;
  border-radius: 5px;
}

.affected-files {
  color: #6c757d;
  font-size: 0.9em;
}

.status-badge {
  width: 55px;
  margin-right: 20px;
  display: inline-block;
}

.message-block {
  margin-bottom: 30px;
  margin-left: 30px;
}

.message-code {
  color: #6c757d;
  font-size: 0.9em;
  margin-left: 4px;
}
</style>