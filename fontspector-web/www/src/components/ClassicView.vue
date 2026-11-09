<script setup lang="ts">
import { SORT_RESULT, state } from '../store';
import { computed, ref } from 'vue';
import { CheckResult, Status, StatusCode } from '../types';
import { renderMarkdown } from '../markdown';
import { CheckSpecificRendering } from '../rendering';

function renderLog(log: Status, id: string, filename: string) {
  let extra_html = "";
  let suppress = false;
  if (log.metadata && id in CheckSpecificRendering) {
    [extra_html, suppress] = CheckSpecificRendering[id](
      log.metadata,
      state.fonts[filename],
    );
    if (suppress) {
      return extra_html;
    }
  }

  return `
    <li>
      <span class="bg-${log.severity} font-weight-bold px-1">${log.severity}</span>:
      <div>${renderMarkdown(log.message || "")}</div>
      ${extra_html}
    </li>
  `;
}
const activeResult = computed(() => {
  return state.lastResults?.find(r => r.check_id === state.activeCheckId);
});

const groupedResults = computed(() => {
  if (!state.lastResults) return {};
  const groups: Record<StatusCode, CheckResult[]> = {} as any;

  // Sort results by severity first
  const sorted = [...state.lastResults].sort((a, b) => {
    return SORT_RESULT[a.worst_status].localeCompare(SORT_RESULT[b.worst_status]);
  });

  for (const res of sorted) {
    if (!groups[res.worst_status]) groups[res.worst_status] = [];
    groups[res.worst_status].push(res);
  }
  return groups;
});

const groupedLogs = computed(() => {
  if (!activeResult.value) return {};
  const logs: Record<string, Status[]> = {};
  for (const log of activeResult.value.subresults) {
    const filename = activeResult.value.filename || 'Family Check';
    if (!logs[filename]) logs[filename] = [];
    logs[filename].push(log);
  }
  return logs;
});

</script>
<template>
  <div class="container-fluid row pt-2">
    <div class="col-5">
      <div class="nav flex-column nav-pills flex-scroll" id="v-pills-tab" role="tablist">
        <template v-for="(group, status) in groupedResults" :key="status">
          <button class="nav-link disabled" :class="'header-' + status"></button>
          <button v-for="result in group" :key="result.check_id" class="nav-link"
            :class="['bg-' + result.worst_status, { active: state.activeCheckId === result.check_id }]" type="button"
            @click="state.activeCheckId = result.check_id">
            {{ result.check_name }}
          </button>
        </template>
      </div>
    </div>
    <div class="col-7">
      <div class="tab-content" id="v-pills-tabContent">
        <div v-if="activeResult" class="tab-pane show active" role="tabpanel">
          <h4>{{ activeResult.check_name }}</h4>
          <p class="text-muted">{{ activeResult.check_id }}</p>
          <div class="rationale" v-html="renderMarkdown(activeResult.check_rationale)"></div>
          <ul class="results list-unstyled">
            <li v-for="(logs, filename) in groupedLogs" :key="filename">
              <b v-if="filename !== 'Family Check'">{{ filename }}</b>
              <ul>
                <li v-for="(log, idx) in logs" :key="idx" v-html="renderLog(log, activeResult.check_id, filename)">
                </li>
              </ul>
            </li>
          </ul>
        </div>
        <div v-else>
          <h4>Select a check result to see details</h4>
        </div>
      </div>
    </div>
  </div>
</template>
<style>
.header-PASS::before {
  content: "✅ Passing checks";
}

.header-SKIP::before {
  content: "⏩ Skipped checks";
}

.header-WARN::before {
  content: "⚠️ Warnings";
}

.header-INFO::before {
  content: "ℹ️ Information";
}

.header-FAIL::before {
  content: "🔥 Failing checks";
}

.header-ERROR::before {
  content: "💥 Errors";
}

.header-FATAL::before {
  content: "💀 Fatal Errors";
}

.bg-PASS {
  background-color: #8df0a8 !important;
}

.bg-SKIP {
  background-color: #acb0ad !important;
}

.bg-WARN {
  background-color: #e0cf9b !important;
}

.bg-FAIL {
  background-color: #e0999b !important;
}

.bg-ERROR {
  background-color: #050505 !important;
  color: #888888 !important;
}

.bg-FATAL {
  background-color: #ff0000 !important;
  color: white !important;
}

.bg-INFO {
  background-color: #bdbcf7 !important;
}

.nav-link+.nav-link {
  margin-top: 5px;
}
</style>