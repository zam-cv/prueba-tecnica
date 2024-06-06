import { BrowserRouter, Routes, Route } from "react-router-dom";

import Layout from "./components/Layout";
import Protected from "./components/Protected";

// pages
import Panel from "./components/Panel";
import Register from "./components/Register";
import Login from "./components/Login";

import "./App.css";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index path="/login" element={<Login />} />
          <Route
            path="/panel"
            element={
              <Protected>
                <Panel />
              </Protected>
            }
          />
          <Route path="/register" element={<Register />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
