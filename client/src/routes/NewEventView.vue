<script setup>
  import { reactive } from 'vue';
  
  import { server } from '../api';
  import { useRouter } from 'vue-router';

  const router = useRouter();

  function getDateText() {
    const date = new Date();
    return date.toISOString();
  }

  const newEventData = reactive({
    id: null,
    startDate: "2025-01-01",
    endDate: "2025-01-01",
    startTime: "12:00",
    endTime: "1:00",
    place: "",
  });

  async function submitNewEventForm(event) {
    event.preventDefault();
    try {
      const newEventRes = await server.newEvent(newEventData);
      router.push("/events")
    } catch (e) {
      console.error(e);
    }

  }
</script>

<template>
  <h1>New Event</h1>
  <form @submit="submitNewEventForm">
    <label for="startDate">Start Date</label>
    <input type="date" name="startDate" v-model="newEventData.startDate">
    <label for="endDate">End Date</label>
    <input type="date" name="endDate" v-model="newEventData.endDate">
    <label for="startTime">Start Time</label>
    <input type="time" name="startTime" v-model="newEventData.startTime">
    <label for="endTime">End Time</label>
    <input type="time" name="endTime" v-model="newEventData.endTime">
    <label for="place">Place</label>
    <input type="text" name="place" v-model="newEventData.place">
    <button type="submit">Submit</button>
  </form>
</template>

<style>
  label {
    display: block;
  }
  input {
    display: block;
  }
</style>
