<script setup lang="ts">
import { EMOJIS, STATUS_LABELS } from '../constants';
import { state, updateResults, resetState } from '../store';
import { useColorMode } from 'bootstrap-vue-next'
import { ref } from 'vue';

const mode = useColorMode();
function toggleView() {
    state.view = state.view === 'classic' ? 'problem' : 'classic';
}

const changeColor = () => {
    mode.value = mode.value === 'dark' ? 'light' : 'dark'
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

<template>
    <nav class="navbar bg-navbar px-5 mb-4">
        <a class="leftarrow text-secondary text-decoration-none" href="#" @click.prevent="resetState">←</a>
        <a class="navbar-brand d-block">
            <img src="/lens.svg" width="60" height="60" class="d-inline-block align-top m-2" alt="" />
            <div class="d-inline-block align-middle">
                Fontspector {{ state.view === 'classic' ? 'Check' : 'Problem' }} Report <br />
                for <span id="font-name">{{ state.currentFontName }}</span>
            </div>
            <div class="fs-6 pl-4 text-muted">
                v{{ state.version }} –
                <a href="https://github.com/fonttools/fontspector/issues">Report tool issues</a>
            </div>

        </a>
        <div id="badges">
            <span v-for="(label, status) in STATUS_LABELS" :key="status" class="mx-2" :title="label">
                {{ EMOJIS[status] }} <span>{{ state.counts[status] }}</span>
            </span>
        </div>
        <button class="btn btn-primary" @click="downloadReport">Download Report</button>
        <div class="form-check form-switch">
            <input class="form-check-input" type="checkbox" role="switch" id="view-switch"
                :checked="state.view === 'classic'" @change="toggleView" />
            <label class="form-check-label" for="view-switch">Classic view</label>
        </div>

        <div class="btn" @click="changeColor">{{ mode === 'dark' ? '☀️' : '🌙' }}</div>
    </nav>
</template>

<style>
[data-bs-theme=light] .bg-navbar {
    background-color: #ebebeb;
}

[data-bs-theme=dark] .bg-navbar {
    background-color: #2c2c2c;
}
</style>
