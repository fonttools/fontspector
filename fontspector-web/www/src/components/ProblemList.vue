<script setup lang="ts">
import { computed, ref } from 'vue';
import { renderMarkdown } from '../markdown';
import { state } from '../store';
import { FixItem, FixRequest, isFontProblem, isGlyphProblem, isTableProblem, Metadata, SubresultWithCheck } from '../types';
import fbWorker from '../workersingleton';
const props = defineProps<{
  resultClass: string,
  fixable: boolean,
  results: any[],
}>();

function firstMetadata(sr: SubresultWithCheck): Metadata | null {
  for (const sub of sr.check.subresults) {
    for (const meta of sub.metadata || []) return meta;
  }
  return null;
}

function checkArea(sr: SubresultWithCheck): string {
  const meta = firstMetadata(sr);
  if (!meta) return sr.check.check_name;
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
function selectAllChildren(event: Event) {
  const checkbox = event.target as HTMLInputElement;
  const details = checkbox.closest("details");
  if (!details) return;
  const childCheckboxes = details.querySelectorAll("input[type=checkbox].individual-fix");
  childCheckboxes.forEach(cb => {
    const checkId = cb.getAttribute("data-checkid");
    const filenames = JSON.parse(cb.getAttribute("data-filenames") || "[]");
    if (checkId) {
      if (checkbox.checked) {
        filenames.forEach((filename: string) => selectedFixRequests.value.add({
          check_id: checkId,
          filename,
        }));
      } else {
        filenames.forEach((filename: string) => selectedFixRequests.value.delete({
          check_id: checkId,
          filename
        }));
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
  // If the status is the same, but only check.filename differs, we want to cluster them together as they likely indicate the same underlying issue across multiple files
  const groups: Record<string, SubresultWithCheck[]> = {};
  for (const res of results) {
    const key = JSON.stringify(res.status) + res.check.check_id;
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

const selectedFixRequests = ref<Set<FixItem>>(new Set());

function toggleFixRequest(checkId: string, filenames: string[], e: Event) {
  for (var filename of filenames) {
    const fixRequest: FixItem = {
      check_id: checkId,
      filename,
      details: null,
    };
    if ((e.target as HTMLInputElement).checked) {
      selectedFixRequests.value.add(fixRequest);
    } else { selectedFixRequests.value.delete(fixRequest); }
  }
}

function baseName(path: string): string {
  return path.split('/').slice(-1)[0];
}

</script>
<template>
  <div>
    <p><span class="resultclass">{{ resultClass }}</span> ({{ results.length }} issues across {{ fileCount(results) }}
      files<span v-if="fixable">,
        fix: <input type="checkbox" @change="selectAllChildren" /></span>) </p>
    <details v-for="(group, area) in groupByArea(results)" :key="area" open="true">
      <summary>{{ area }} <span v-if="fixable">(Fix {{ group.length }} issues: <input type="checkbox"
            @change="selectAllChildren" />)</span>
      </summary>

      <details v-for="(resGroup, idx) in collateFiles(group)" :key="idx">
        <!-- Within this group, status and check are the same -->
        <summary>
          <span class="checkname">{{ resGroup[0].check.check_name }}</span>
          <span v-if="resGroup.length > 1" class="affected-files"> (Affects {{ resGroup.length }} files)</span><span
            v-if="resGroup.length == 1" class="affected-files"> (Affects {{ baseName(resGroup[0].check.filename ||
              "whole family") }})</span>
          <span v-if="fixable">
            Fix:
            <input type="checkbox" class="individual-fix" :data-checkid="resGroup[0].check.check_id"
              :data-filenames="JSON.stringify(resGroup.map(r => r.check.filename || 'Family Check'))" :checked="selectedFixRequests.has(
                { check_id: resGroup[0].check.check_id, filename: resGroup[0].check.filename || 'Family Check' })"
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
    <div class="pt-3" v-if="fixable"><button class="btn btn-primary" @click="fixAndDownload"
        :disabled="selectedFixRequests.size == 0">Fix
        {{ selectedFixRequests.size }} Selected Issues</button></div>
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
  background-color: #f8f9fa;
  padding: 10px;
  border-radius: 5px;
}

.affected-files {
  color: #6c757d;
  font-size: 0.9em;
}

table {
  width: 100%;
  margin-top: 10px;
  background-color: aliceblue;
}

table td,
th {
  padding: 6px;
  border: 2px solid white;
}

table th {
  background-color: #007bff;
  color: white;
  text-align: center;
}

table tr:nth-child(even) {
  background-color: #d4e5f6;
}
</style>