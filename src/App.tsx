import { Outlet } from "react-router-dom";
import { GlobalEventHandler } from "./components/GlobalEventHandler";

function App() {
  return (
    <GlobalEventHandler>
      <Outlet />
    </GlobalEventHandler>
  );
}

export default App;
