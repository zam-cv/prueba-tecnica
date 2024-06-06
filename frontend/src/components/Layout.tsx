import { Outlet } from "react-router-dom";
import Header from "./Header";

export default function Layout() {
  const isAuthenticated = false;

  return (
    <div className="h-full grid grid-rows-[auto_1fr] overflow-hidden">
      <div>{isAuthenticated ? <Header /> : <div></div>}</div>
      <div className="overflow-auto">
        <Outlet />
      </div>
    </div>
  );
}
