<script setup>
import { store } from "../store";
import { server } from "../api";

let events = store.ownedEvents;

server.getOwnedEvents();

function deleteButtonHandler(eventId) {
  server.deleteEvent(eventId);
}
</script>

<template>
  <h1>Events</h1>
  <ol v-if="!store.ownedEvents || store.ownedEvents.length">
    <li v-for="e in store.ownedEvents" :key="e.id">
      <pre>
        {{ e }}
      </pre>
      <button @click="deleteButtonHandler(e.id)">Delete</button>
    </li>
  </ol>
  <p v-else>No Events have been added yet</p>
</template>
