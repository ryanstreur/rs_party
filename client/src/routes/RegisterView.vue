<script setup>
  import {ref} from 'vue'

  import {ax} from '../api';

  const email_input = ref('')
  const name_input = ref('')
  const password_input = ref('')
  const password_confirmation_input = ref('')

  async function submit_registration_form(event) {
    event.preventDefault();
    console.log('submitted form')
  }

</script>

<template>
  <h1>Register</h1>
  <form @submit="submit_registration_form">
    <label for="email">Email Address</label>
    <input name="email" type="email" v-model="email_input">
    <label for="name">Name</label>
    <input name="name" type="text" v-model="name_input">
    <label for="password">Password</label>
    <input name="password" type="password" v-model="password_input">
    <label for="password_confirmation">Confirm Password</label>
    <input name="password_confirmation" type="password" v-model="password_confirmation_input">
    <button type="submit">Submit</button>
  </form>

  <p>
    email: {{ email_input }}
  </p>
  <p>
    name: {{ name_input }}
  </p>

</template>
