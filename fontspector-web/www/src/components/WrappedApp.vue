<template>
    <div id="app-container">
        <!-- Start Modal -->
        <div v-if="state.view === 'start'" class="modal d-block" tabindex="-1">
            <StartModal />
        </div>

        <!-- Results View -->
        <div v-if="state.view === 'classic' || state.view === 'problem'" id="results-container">
            <Navbar />

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
import { state, updateResults, resetState } from '../store';
import ProblemView from './ProblemView.vue';
import StartModal from './StartModal.vue';
import ErrorModal from './ErrorModal.vue';
import ListChecks from './ListChecks.vue';
import ClassicView from './ClassicView.vue';
import fbWorker, { postToWorker } from '../workersingleton';
import { ReplyMessage } from '../types';
import JSZip from "jszip";
import { useToast } from 'bootstrap-vue-next'
import Navbar from './Navbar.vue';

const { create } = useToast()
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
        if (data.download) {
            const zipBlob = new Blob([data.zipfile as ArrayBufferView<ArrayBuffer>], { type: 'application/zip' });
            const url = URL.createObjectURL(zipBlob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'fontspector-fix.zip';
            a.click();
        }
        // Rerun the tests
        let contents = JSZip.loadAsync(data.zipfile).then((zip) => {
            const files: Record<string, Uint8Array> = {};
            const promises: any[] = [];
            zip.forEach((relativePath, zipEntry) => {
                // Skip the log file
                if (!zipEntry.dir && !relativePath.endsWith("fix_log.txt")) {
                    const promise = zipEntry.async("uint8array").then((content) => {
                        files[relativePath] = content;
                    });
                    promises.push(promise);
                }
            });
            return Promise.all(promises).then(() => files);
        }).then((files) => {
            if (data.download) {
                create({ title: "Fix applied!", body: "Fixes have been applied, and the new fonts downloaded to your downloads folder. Check status has been updated.", variant: "success" });
            }
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

</script>
