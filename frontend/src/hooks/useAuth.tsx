import { useEffect, useState, useContext } from "react";
import { AuthContext, AuthContextType } from "../contexts/AuthContext";
import { NavigateFunction } from "react-router-dom";
import api from "../utils/api";

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const auth = useProvideAuth();
  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextType {
  return useContext(AuthContext);
}

export function useProvideAuth() {
  const [isLoading, setIsLoading] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem("token");

    if (token) {
      api.auth
        .verify()
        .then(() => {
          setIsAuthenticated(true);
          setIsLoading(false);
        })
        .catch(() => {
          setIsAuthenticated(false);
          setIsLoading(false);
        });
    } else {
      setIsAuthenticated(false);
      setIsLoading(false);
    }
  }, []);

  function signout() {
    localStorage.removeItem("token");
    setIsAuthenticated(false);
  }

  function signin(email: string, password: string, navigate: NavigateFunction) {
    setIsLoading(true);

    api.auth
      .signin(email, password)
      .then((data) => {
        localStorage.setItem("token", data);
        setIsAuthenticated(true);
        setIsLoading(false);
        navigate("/rooms");
      })
      .catch((error) => {
        setIsLoading(false);
        setIsAuthenticated(false);
        console.error(error);
      });
  }

  return { isLoading, isAuthenticated, signout, signin };
}
