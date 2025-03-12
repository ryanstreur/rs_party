import axios, { type AxiosInstance } from "axios";

import { type RegistrationBody } from "./model";

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
}
