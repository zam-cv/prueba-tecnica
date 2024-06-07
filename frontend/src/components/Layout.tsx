import { useAuth } from "../hooks/useAuth";
import { Outlet } from "react-router-dom";
import Header from "./Header";

export default function Layout() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="h-full grid grid-rows-[auto_1fr] overflow-hidden">
      <div>{isAuthenticated ? <Header /> : <div></div>}</div>
      <div className="overflow-auto">
        <Outlet />
      </div>
    </div>
  );
}
