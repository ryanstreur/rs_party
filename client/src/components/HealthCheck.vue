<script setup lang="ts">
import { ref, type Ref } from "vue";
import { Server } from "../api";

const server = new Server();
let last_successful_hc: Ref<string, string> = ref("None");
let last_failed_hc: Ref<string, string> = ref("None");

async function checkHealth() {
  try {
    const res = await server.hc();
    console.log("API Healthcheck succeeded", res);
    last_successful_hc.value = Date();
  } catch (err) {
    console.error("API Healthcheck failed", err);
    last_failed_hc.value = Date();
  }
}

setInterval(checkHealth, 5000);
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
    </tbody>
  </table>
</template>
