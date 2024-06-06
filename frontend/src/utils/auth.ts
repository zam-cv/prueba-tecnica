interface Config {
  withCredentials: boolean;
  headers: {
    Authorization: string | null;
  };
}

export function getConfig(): Config {
  return {
    withCredentials: true,
    headers: {
      Authorization: localStorage.getItem("token"),
    }
  }
}