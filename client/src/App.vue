<script setup lang="ts">
  import  HealthCheck  from "./components/HealthCheck.vue";
  import { store } from "./store";
</script>

<template>
  <nav>
    <ul>
      <li>
        <RouterLink to="/">Home</RouterLink>
      </li>
      <div v-if="store.authenticated">
        <li>
          <button @click="store.logOut">
            Log Out
          </button>
        </li>
      </div>
      <div v-else>
        <li>
          <RouterLink to="/about">About</RouterLink>
        </li>
        <li>
          <RouterLink to="/login">Log In</RouterLink>
        </li>
        <li>
          <RouterLink to="/register">Register</RouterLink>
        </li>
      </div>
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
