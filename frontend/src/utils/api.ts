import { post } from "./methods";

export default {
  auth: {
    register: (username: string, email: string, password: string): Promise<void> => {
      return post("/auth/register", { username, email, password }, false);
    },
  },
};
