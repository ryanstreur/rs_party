<script setup>
  import { store } from '../store';
  import { server } from '../api';

  let events = store.ownedEvents;

  server.getOwnedEvents()
</script>

<template>
  <h1>Events</h1>
  <ol>
    <li v-for="e in store.ownedEvents">
      <pre>
        {{e}}
      </pre>
    </li>
  </ol>
</template>
