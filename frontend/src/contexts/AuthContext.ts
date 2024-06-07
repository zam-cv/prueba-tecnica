import { createContext } from 'react';
import { NavigateFunction } from 'react-router';

export type AuthContextType = {
  isLoading: boolean;
  isAuthenticated: boolean;
  signout: () => void;
  signin: (email: string, password: string, navigate: NavigateFunction) => void;
}

export const AuthContext = createContext<AuthContextType>({
  isLoading: true,
  isAuthenticated: false,
  signout: () => {},
  signin: () => {},
});