<script setup lang="ts">
import { ref, type Ref } from "vue";
import { Server } from "../api";
import { store } from "../store";

const server = new Server();
let last_successful_hc: Ref<string, string> = ref("None");
let last_failed_hc: Ref<string, string> = ref("None");
checkHealth();
let healthCheckInterval = setInterval(checkHealth, store.healthCheckTimeout);

async function checkHealth() {
  try {
    const res = await server.hc();
    last_successful_hc.value = Date();
  } catch (err) {
    console.error("API Healthcheck failed", err);
    last_failed_hc.value = Date();
  }
}
</script>

<template>
  <table>
    <tbody>
      <tr>
        <th>Last Successful Health Check</th>
        <td>{{ last_successful_hc }}</td>
      </tr>
      <tr>
        <th>Last Failed Health Check</th>
        <td>{{ last_failed_hc }}</td>
      </tr>
      <tr>
        <th>Session Key</th>
        <td>{{ store.sessionKey ? store.sessionKey : "None" }}</td>
      </tr>
      <tr>
        <th>User</th>
        <td>{{ store.authenticatedUser ? store.authenticatedUser : "None" }}</td>
      </tr>
    </tbody>
  </table>
</template>


