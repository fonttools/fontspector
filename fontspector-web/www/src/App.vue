<template>
  <div id="app-container">
    <!-- Start Modal -->
    <div v-if="state.view === 'start'" class="modal d-block" tabindex="-1">
      <StartModal />
    </div>

    <!-- Results View -->
    <div v-if="state.view === 'classic' || state.view === 'problem'" id="results-container">
      <nav class="navbar navbar-light bg-light">
        <a class="leftarrow text-secondary" href="#" @click.prevent="resetState">&lt;</a>
        <a class="navbar-brand d-block" href="#" @click.prevent="resetState">
          <img src="/lens.svg" width="60" height="60" class="d-inline-block align-top" alt="" />
          <div class="d-inline-block align-middle">
            Fontspector {{ state.view === 'classic' ? 'Check' : 'Problem' }} Report <br />
            for <span id="font-name">{{ state.currentFontName }}</span>
          </div>
        </a>
        <div id="badges">
          <span v-for="(label, status) in STATUS_LABELS" :key="status" class="ml-4 mr-4" :title="label">
            {{ EMOJIS[status] }} <span>{{ state.counts[status] }}</span>
          </span>
        </div>
        <div id="download">
          <button class="btn btn-primary" @click="downloadReport">Download</button>
        </div>
        <div class="form-check form-switch">
          <input class="form-check-input" type="checkbox" role="switch" id="view-switch"
            :checked="state.view === 'classic'" @change="toggleView" />
          <label class="form-check-label" for="view-switch">Classic view</label>
        </div>
      </nav>
      <p class="fs-6 bg-light pl-4 text-muted">
        Fontspector version {{ state.version }}
      </p>

      <!-- Classic View Content -->
      <ClassicView v-if="state.view === 'classic'" />

      <!-- Problem View Content -->
      <ProblemView v-if="state.view === 'problem'" />
    </div>

    <ListChecks v-if="state.view === 'listChecks'" />
    <ErrorModal v-if="state.error" />
  </div>
</template>

<script setup lang="ts">
import { state, updateResults, resetState } from './store';
import { EMOJIS, STATUS_LABELS } from './constants';
import ProblemView from './components/ProblemView.vue';
import StartModal from './components/StartModal.vue';
import ErrorModal from './components/ErrorModal.vue';
import ListChecks from './components/ListChecks.vue';
import ClassicView from './components/ClassicView.vue';
import fbWorker, { postToWorker } from './workersingleton';
import { ReplyMessage } from './types';
import JSZip from "jszip";

// @ts-ignore
let hbjs = window["hbjs"];



fbWorker.onmessage = (event: any) => {
  const data: ReplyMessage = event.data;
  console.log("Worker message:", data);
  if (data.id == "ready") {
    state.loading = false;
    state.version = data.version;
    state.allChecks = data.checks;

    if (window.location.hash) {
      const checkName = window.location.hash.substring(1);
      if (checkName in state.allChecks) {
        state.view = 'listChecks';
        // TODO: scroll to it
      }
    }
  } else if (data.id == "fix_result") {
    console.log("Received fix result from worker, preparing download...");
    const zipBlob = new Blob([data.zipfile], { type: 'application/zip' });
    const url = URL.createObjectURL(zipBlob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'fontspector-fix.zip';
    a.click();
    // Rerun the tests
    let contents = JSZip.loadAsync(data.zipfile).then((zip) => {
      const files: Record<string, Uint8Array> = {};
      const promises: any[] = [];
      zip.forEach((relativePath, zipEntry) => {
        if (!zipEntry.dir) {
          const promise = zipEntry.async("uint8array").then((content) => {
            files[relativePath] = content;
          });
          promises.push(promise);
        }
      });
      return Promise.all(promises).then(() => files);
    }).then((files) => {
      postToWorker({
        id: "run_checks",
        files,
        profile: state.selectedProfile,
        loglevels: state.logLevel,
        fulllists: state.fullLists
      });
    });

  } else if (data.id == "name") {
    state.currentFontName = data.name;
  } else if (data.id == "error") {
    state.error = data.error.toString();
    state.loading = false;
  } else if (data.id == "check_result") {
    updateResults(data.results);
    state.view = 'problem';
  }
};

function toggleView() {
  state.view = state.view === 'classic' ? 'problem' : 'classic';
}


function downloadReport() {
  const html = document.documentElement.outerHTML;
  const blob = new Blob([html], { type: 'text/html' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'fontspector-report.html';
  a.click();
}
</script>

<style>
.flex-scroll {
  overflow-y: auto;
  height: calc(100vh - 150px);
}

.modal {
  background: rgba(0, 0, 0, 0.5);
}

.nav-link {
  cursor: pointer;
  margin-bottom: 2px;
}

.nav-link.active {
  border: 2px solid black;
}

#dropzone-container {
  border: 2px dashed #ccc;
  padding: 20px;
  margin: 20px 0;
}
</style>
