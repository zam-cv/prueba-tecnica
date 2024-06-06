import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

export default function Protected({ children }: { children: React.ReactNode }) {
  const navigate = useNavigate();
  const isAuthenticated = false;

  useEffect(() => {
    if (!isAuthenticated) {
      navigate("/login");
    }
  }, [isAuthenticated, navigate]);

  return <>{children}</>;
}