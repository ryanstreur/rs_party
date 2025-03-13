import axios, { type AxiosInstance } from "axios";

import { type LoginBody, type RegistrationBody } from "./model";
import { store } from "./store";

const BASE_URL = "http://localhost:8080/";

export class Server {
  private ax: AxiosInstance;

  constructor() {
    this.ax = axios.create({
      baseURL: BASE_URL,
    });
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
    await this.getUserSelf();
    return loginRes;
  }

  async getUserSelf() {
    const userSelfRes = await this.ax.get("/user/self", {
      headers: {
        Authorization: `Bearer: ${store.sessionKey}`,
      },
    });

    store.setUser(userSelfRes.data);
    return userSelfRes.data;
  }
}

export const server = new Server();
