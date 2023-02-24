import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Outlet, RouterProvider } from "react-router-dom";
import { router } from "./router";
import { GlobalEventHandler } from "./components/GlobalEventHandler";

function App() {
  return (
    <GlobalEventHandler>
      <Outlet/>
    </GlobalEventHandler>
  );
}

export default App;
