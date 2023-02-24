import { createBrowserRouter } from "react-router-dom";
import App from "../App";
import { NewConfigPage } from "../pages/new-config";
import { OpenConfigPage } from "../pages/open-config";

export const router = createBrowserRouter([
    {
        path: "/",
        element: <App/>,
        children: [
            {
                path: "/new-config",
                element: <NewConfigPage/>,
            },
            {
                path: "/open-config",
                element: <OpenConfigPage/>,
            },
        ]
    }
])