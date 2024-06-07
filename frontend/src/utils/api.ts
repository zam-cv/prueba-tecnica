import { post, get } from "./methods";

export default {
  auth: {
    register: (username: string, email: string, password: string): Promise<void> => {
      return post("/auth/register", { username, email, password }, false);
    },
    signin: (email: string, password: string): Promise<string> => {
      return post("/auth/signin", { email, password }, false);
    },
    verify: (): Promise<void> => {
      return get("/auth/verify");
    }
  },
};
