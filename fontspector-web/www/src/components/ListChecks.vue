<script setup lang="ts">
import { renderMarkdown } from '../markdown';
import { state } from '../store';
</script>
<template>
  <div>
    <nav class="navbar navbar-expand-lg navbar-light bg-light">
      <div class="container-fluid">
        <a class="leftarrow text-secondary" href="#" @click.prevent="state.view = 'start'">&lt;</a>
        All the Fontspector Checks!
      </div>
    </nav>
    <div class="container">
      <div v-for="(check, id) in state.allChecks" :key="id" class="card my-4">
        <div class="card-header"><code>{{ id }}</code></div>
        <div class="card-body">
          <h2>{{ check.description }}</h2>
          <div v-html="renderMarkdown(check.rationale)"></div>
          <table class="table">
            <tr v-if="check.proposal && check.proposal.length">
              <th>More info</th>
              <td>
                <ul v-if="check.proposal.length > 1">
                  <li v-for="p in check.proposal" :key="p"><a :href="p" target="_blank">{{ p }}</a></li>
                </ul>
                <a v-else :href="check.proposal[0]" target="_blank">{{ check.proposal[0] }}</a>
              </td>
            </tr>
            <tr>
              <th>Sections</th>
              <td><span v-for="s in check.sections" :key="s" class="badge badge-pill badge-primary mr-2">{{ s
                  }}</span>
              </td>
            </tr>
            <tr>
              <th>Profiles</th>
              <td><span v-for="p in check.profiles" :key="p" class="badge badge-pill badge-primary mr-2">{{ p
                  }}</span>
              </td>
            </tr>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>