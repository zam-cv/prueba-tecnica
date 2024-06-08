import { post, get } from "./methods";

export interface Room {
  id: number;
  title: string;
  description: string;
  front_image: string;
  duration: number;
}

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
  rooms: {
    getRooms: (): Promise<[number, string, string, string, number][]> => {
      return get("/rooms");
    }
  }
};
