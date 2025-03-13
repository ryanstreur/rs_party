import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { server } from './api';
import { store } from './store';
import { router } from './router';

if (store.authenticated) {
  server.getUserSelf();
}

createApp(App).use(router).mount("#app");
