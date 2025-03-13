import {reactive} from 'vue';
import type { User, Event } from './model';
import type { AxiosError } from 'axios';

let storedSessionKey = localStorage.getItem("sessionKey");

export const store = reactive({
  authenticated: !!storedSessionKey,
  authenticatedUser: null as User | null,
  sessionKey: storedSessionKey,
  healthCheckTimeout: 10000,
  ownedEvents: null as model.Event[] | null,
  mostRecentAxiosError: null as any | null,
  setAuthenticated(newValue: boolean) {
    this.authenticated = newValue;
  },
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
  },
  setOwnedEvents(newEvents: Event[]) {
    this.ownedEvents = newEvents;
  },
  setMostRecentAxiosError(mre: any) {
    this.mostRecentAxiosError = mre;
  }
})
