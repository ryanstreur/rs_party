<script setup lang="ts">
  import { Server, ax } from './api';
  import { ref } from 'vue';
  
  const server = new Server(ax);
  let last_successful_hc = ref(0);
  let last_failed_hc = ref(0);

  setInterval(async () => {
    try {
      const res = await server.hc();
      console.log("API Healthcheck succeeded", res)
      last_successful_hc.value = Date.now()
    } catch (err) {
      console.error("API Healthcheck failed", err);
      last_failed_hc.value = Date.now()
    }
  }, 5000)
</script>

<template>
  <nav>
    <ul>
      <li>
        <RouterLink to="/">Home</RouterLink>
      </li>
      <li>
        <RouterLink to="/about">About</RouterLink>
      </li>
      <li>
        <RouterLink to="/register">Register</RouterLink>
      </li>
    </ul>
  </nav>
  <main>
    <RouterView></RouterView>
  </main>
  <footer>
    <table>
      <tr>
        <th>Last Successful Health Check</th>
        <th>Last Failed Health Check</th>
      </tr>
      <tr>
        <td>{{ last_successful_hc }}</td>
        <td>{{ last_failed_hc }}</td>
      </tr>
    </table>
  </footer>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
