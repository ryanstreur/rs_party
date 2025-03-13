import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { createRouter, createWebHistory } from "vue-router";

import HomeView from "./routes/HomeView.vue";
import AboutView from "./routes/AboutView.vue";
import LoginView from "./routes/LoginView.vue";
import RegisterView from "./routes/RegisterView.vue";

import { server } from './api';
import { store } from './store';

const routes = [
  { path: "/", component: HomeView },
  { path: "/about", component: AboutView },
  { path: "/login", component: LoginView },
  { path: "/register", component: RegisterView },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

if (store.authenticated) {
  server.getUserSelf();
}

createApp(App).use(router).mount("#app");
