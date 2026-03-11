<script setup lang="ts">
import { ref } from 'vue';
import { renderMarkdown } from '../markdown';
import { SEVERITY_COLOR } from '../constants';
import { FixItem, FixRequest, isFontProblem, isGlyphProblem, isTableProblem, Metadata, SubresultWithCheck } from '../types';
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
// Cluster check results affecting multiple files together
function collateFiles(results: SubresultWithCheck[]): SubresultWithCheck[][] {
  // Group by check_id + status content (message, code, severity)
  // This way:
  // - Identical issues across multiple files get grouped together
  // - Different issues for the same file remain separate
  // - Multiple subresults for the same issue in the same file also get grouped
  const groups: Record<string, SubresultWithCheck[]> = {};
  for (const res of results) {
    // Create a key based on the check and the actual issue content
    const key = JSON.stringify({
      check_id: res.check.check_id,
      message: res.status.message,
      code: res.status.code,
      severity: res.status.severity,
    });
    if (!groups[key]) groups[key] = [];
    groups[key].push(res);
  } 
  return Object.values(groups);
}
function fixAndDownload() {
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
    <details v-for="([area, group], idx) of Object.entries(groupByArea(results)).sort(([a], [b]) => a.localeCompare(b))" :key="idx" open="true">
      <summary class="mb-2">{{ area }} <span v-if="fixable && collateFiles(group).length > 1" class="float-end">Fix {{
        group.length
          }} issues:
          <input type="checkbox" @change="selectAllChildren" /></span>
      </summary>

      <details v-for="(resGroup, idx) in collateFiles(group)" :key="idx" class="mb-3">
        <!-- Within this group, status and check are the same -->
        <summary>
          <span :class="`badge status-badge text-bg-${SEVERITY_COLOR[resGroup[0].status.severity]} m-1`">{{
            resGroup[0].status.severity
            }}</span>
          <span class="checkname">{{ resGroup[0].check.check_name }}</span>
          <span v-if="resGroup.length > 1" class="affected-files"> (Affects {{ resGroup.length }} files)</span><span
            v-if="resGroup.length == 1" class="affected-files"> (Affects {{ baseName(resGroup[0].check.filename ||
              "whole family") }})</span>
          <span v-if="fixable" style="float: right;">
            Fix:
            <input type="checkbox" class="individual-fix" :data-checkid="resGroup[0].check.check_id"
              :data-filenames="JSON.stringify(resGroup.map(r => r.check.filename || 'Family Check'))"
              :checked="selectedFixRequests.has(makeFixKey(resGroup[0].check.check_id, resGroup[0].check.filename || 'Family Check'))"
              @change="(e) => { toggleFixRequest(resGroup[0].check.check_id, resGroup.map(r => r.check.filename || 'Family Check'), e) }" />
          </span>
        </summary>
        <p class="affected-files" v-if="resGroup.length > 1">Affects
        <ul>
          <li v-for='file in resGroup.map(r => r.check.filename || "Family Check")'>
            {{ file }}
          </li>
        </ul>
        </p>
        <p class="rationale" v-html="renderMarkdown(resGroup[0].check.check_rationale)"></p>
        <p class="message" v-html="renderMarkdown(resGroup[0].status.message || '')"></p>
      </details>
    </details>
    <div class="clearfix">
      <div class="float-end" v-if="fixable"><button class="btn btn-primary" @click="fixAndDownload"
          :disabled="selectedFixRequests.size == 0">Fix
          {{ selectedFixRequests.size }} Selected Issues</button></div>
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
</style>