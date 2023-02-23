import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { GlobalEventHandler } from "./components/GlobalEventHandler";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <GlobalEventHandler>
      <App />
    </GlobalEventHandler>
  </React.StrictMode>
);
