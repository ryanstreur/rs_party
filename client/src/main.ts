import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { createMemoryHistory, createRouter } from "vue-router";

import HomeView from "./routes/HomeView.vue";
import AboutView from "./routes/AboutView.vue";

const routes = [
  { path: "/", component: HomeView },
  { path: "/about", component: AboutView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
