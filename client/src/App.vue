<script setup lang="ts">
  import  HealthCheck  from "./components/HealthCheck.vue";
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
    <HealthCheck />
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
