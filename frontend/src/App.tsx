import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import Panel from "./components/Panel";
import "./App.css";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Panel />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
