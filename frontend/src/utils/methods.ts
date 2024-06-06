import { getConfig } from "./auth";
import { API_URL } from "./constants";
import axios from "axios";

export async function post<T, B>(path: string, body: B, withConfig = true) {
  return axios
    .post(`${API_URL}${path}`, body, withConfig ? getConfig() : undefined)
    .then(({ data }: { data: T }) => data);
}
