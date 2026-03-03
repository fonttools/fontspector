<script setup lang="ts">
import { computed } from "vue";
import {
  CheckResult,
  GlyphProblem,
  TableProblem,
  FontProblem,
  Metadata,
  Status,
  isGlyphProblem,
  isTableProblem,
  isFontProblem,
  SubresultWithCheck,
} from "../types";
import { state } from "../store";
import { renderMarkdown } from '../markdown';
import ProblemList from "./ProblemList.vue";

function invertResults(cr: CheckResult[]): SubresultWithCheck[] {
  const results: SubresultWithCheck[] = [];
  for (const res of cr) {
    if (res.worst_status === "PASS" || res.worst_status === "SKIP") continue;
    for (const sub of res.subresults) {
      results.push({ status: sub, check: res });
    }
  }
  return results;
}

function splitFixable(results: SubresultWithCheck[]): [SubresultWithCheck[], SubresultWithCheck[]] {
  const fixable: SubresultWithCheck[] = [];
  const unfixable: SubresultWithCheck[] = [];
  for (const res of results) {
    if (res.check.hotfix_available) {
      fixable.push(res);
    } else {
      unfixable.push(res);
    }
  }
  return [fixable, unfixable];
}
const fixableResults = computed(() => {
  return splitFixable(invertResults(state.lastResults || []))[0];
});

const unfixableResults = computed(() => {
  return splitFixable(invertResults(state.lastResults || []))[1];
});


</script>
<template>
  <div class="container" id="problem-container">
    <div v-if="fixableResults.length > 0">
      <ProblemList id="fixable" :results="fixableResults" resultClass="Fixable automatically" :fixable="true" />
    </div>

    <hr>

    <ProblemList v-if="unfixableResults.length > 0" :results="unfixableResults" resultClass="Requires manual fix"
      :fixable="false" />
  </div>
</template>

<style></style>
