import axios, { type AxiosInstance } from 'axios';

const BASE_URL = 'http://localhost:8080/';

export const ax = axios.create({
  baseURL: BASE_URL
});

export class Server {
  constructor(
    private ax: AxiosInstance
  ) {}

  hc() {
    return this.ax.get('/hc');
  }
}