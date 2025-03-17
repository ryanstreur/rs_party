<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

import { server } from "../api";

const router = useRouter();
const email_input = ref("");
const name_input = ref("");
const password_input = ref("");
const password_confirmation_input = ref("");

async function submit_registration_form(event) {
  event.preventDefault();
  console.log("submitted form");
  // TODO: Check passwords match
  // TODO: Email validation
  // TODO: Password Requirements
  try {
    const res = await server.postRegistration({
      email: email_input.value,
      password: password_input.value,
      name: name_input.value,
    });

    console.log("response successful", res.data);
    localStorage.setItem("sessionKey", res.data);
    router.push({path: "/"});
  } catch (e) {
    console.error("registration failed", e);
  }
}
</script>

<template>
  <h1>Register</h1>
  <form @submit="submit_registration_form">
    <label for="email">Email Address</label>
    <input name="email" type="email" v-model="email_input" />
    <label for="name">Name</label>
    <input name="name" type="text" v-model="name_input" />
    <label for="password">Password</label>
    <input name="password" type="password" v-model="password_input" />
    <label for="password_confirmation">Confirm Password</label>
    <input
      name="password_confirmation"
      type="password"
      v-model="password_confirmation_input"
    />
    <button type="submit">Submit</button>
  </form>
</template>
