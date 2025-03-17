import axios, { type AxiosInstance } from "axios";

import { type Event, type LoginBody, type RegistrationBody } from "./model";
import { store } from "./store";

const BASE_URL = "http://localhost:8080/";

export class Server {
  private ax: AxiosInstance;

  constructor() {
    this.ax = axios.create({
      baseURL: BASE_URL,
    });

    this.ax.interceptors.response.use(
      (r) => r,
      (e) => {
        console.error(e);

        if (e.status == 401) {
          console.log("unauthorized");
          store.logOut();
          router.push({ path: "/" });
        }
        store.setMostRecentAxiosError(e);
      }
    );
  }

  hc() {
    return this.ax.get("/hc");
  }

  postRegistration(body: RegistrationBody) {
    return this.ax.post("/register", body);
  }

  async postLogin(body: LoginBody) {
    const loginRes = await this.ax.post("/login", body);
    store.setSessionKey(loginRes.data);
    store.setAuthenticated(true);
    await this.getUserSelf();
    return loginRes;
  }

  async getUserSelf() {
    try {
      const userSelfRes = await this.ax.get("/user/self", {
        headers: {
          Authorization: `Bearer: ${store.sessionKey}`,
        },
      });

      store.setUser(userSelfRes.data);
      return userSelfRes.data;
    } catch (e) {
      throw e;
    }
  }

  async newEvent(event: Event): Promise<Event> {
    console.log(store.sessionKey);
    const newEventRes = await this.ax.post("/event", event, {
      headers: { Authorization: "Bearer: " + store.sessionKey },
    });

    this.getOwnedEvents();

    return newEventRes.data;
  }

  async getOwnedEvents(): Promise<Event[]> {
    const eventsResponse = await this.ax.get("/event/own", {
      headers: { Authorization: "Bearer: " + store.sessionKey },
    });

    store.setOwnedEvents(eventsResponse.data);
    return eventsResponse.data;
  }

  async updateEvent(event: Event): Promise<Event> {
    const updatedEventRes = await this.ax.patch("/event", event, {
      headers: { Authorization: "Bearer: " + store.sessionKey },
    });

    return updatedEventRes.data as Event;
  }

  async deleteEvent(eventId: number) {
    await this.ax.delete(`/event/${eventId}`, {
      headers: { Authorization: "Bearer: " + store.sessionKey }
    });

    await this.getOwnedEvents();
  }
}

export const server = new Server();
