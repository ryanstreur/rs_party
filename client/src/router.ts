import { createRouter, createWebHistory } from "vue-router";

import HomeView from "./routes/HomeView.vue";
import AboutView from "./routes/AboutView.vue";
import LoginView from "./routes/LoginView.vue";
import RegisterView from "./routes/RegisterView.vue";
import EventsView from "./routes/EventsView.vue";
import NewEventView from "./routes/NewEventView.vue";


const routes = [
  { path: "/", component: HomeView },
  { path: "/about", component: AboutView },
  { path: "/login", component: LoginView },
  { path: "/register", component: RegisterView },
  { path: "/events", component: EventsView },
  { path: "/events/new", component: NewEventView },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
