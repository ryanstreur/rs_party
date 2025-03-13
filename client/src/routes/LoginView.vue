<script setup>
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';

  import { server } from '../api';
  import { store } from '../store';

  const router = useRouter();

  const loginData = ref({
    email: "",
    password: ""
  })

  async function submitLoginForm(event) {
    event.preventDefault();
    try {
      const res = await server.postLogin(loginData.value);
      store.setSessionKey(res.data);
      router.push("/");
    } catch (e) {
      console.error(e);
    }
  }

</script>

<template>
  <h1>
    Login
  </h1>
  <form @submit="submitLoginForm">
    <label for="email">Email Address</label>
    <input name="email" type="email" v-model="loginData.email" />
    <label for="password">Password</label>
    <input name="password" type="password" v-model="loginData.password" />
    <button type="submit">Log In</button>
  </form>

</template>
