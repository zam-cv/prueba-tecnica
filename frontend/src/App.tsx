import { AuthProvider } from "./hooks/useAuth";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import Layout from "./components/Layout";

// security
import Protected from "./components/Protected";
import Unprotected from "./components/Unprotected";

// pages
import Panel from "./components/Panel";
import Register from "./components/Register";
import Login from "./components/Login";

import "./App.css";

function App() {
  return (
    <AuthProvider>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route
              index
              element={
                <Unprotected>
                  <Login />
                </Unprotected>
              }
            />
            <Route
              path="/panel"
              element={
                <Protected>
                  <Panel />
                </Protected>
              }
            />
            <Route
              path="/register"
              element={
                <Unprotected>
                  <Register />
                </Unprotected>
              }
            />
          </Route>
        </Routes>
      </BrowserRouter>
    </AuthProvider>
  );
}

export default App;
