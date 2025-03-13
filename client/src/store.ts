import {reactive} from 'vue';
import type { User } from './model';

let storedSessionKey = localStorage.getItem("sessionKey");

export const store = reactive({
  authenticated: !!storedSessionKey,
  authenticatedUser: null as User | null,
  sessionKey: storedSessionKey,
  healthCheckTimeout: 10000,
  setSessionKey(newKey: string) {
    this.sessionKey = newKey
    localStorage.setItem("sessionKey", newKey);
  },
  logOut() {
    this.sessionKey = null;
    this.authenticated = false;
    this.authenticatedUser = null;
    localStorage.removeItem("sessionKey");
  },
  setUser(newUser: User) {
    this.authenticatedUser = newUser;
  }
})
