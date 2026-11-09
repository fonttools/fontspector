<script setup lang="ts">
import { state } from '../store';
</script>
<template>
  <div class="modal d-block" tabindex="-1">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">An error occurred</h5>
          <button type="button" class="close" @click="state.error = null">
            <span>&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="alert alert-danger" role="alert">
            <pre>{{ state.error }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>