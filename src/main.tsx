import React from "react";
import ReactDOM from "react-dom/client";
import Root from "./Root";
import Fleet from "./routes/fleet";
import Ship from "./routes/ship";
import System from "./routes/system";
import Waypoint from "./routes/waypoint";
import Faction from "./routes/faction";
import "./index.css";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root/>,
    errorElement: <Root/>,
    children: [
      {
        path: "fleet",
        element: <Fleet/>,
      },
      {
        path: "fleet/:shipId",
        element: <Ship/>
      },
      {
        path: "system/:systemId",
        element: <System/>
      },
      {
        path: "system/:systemId/:waypointId",
        element: <Waypoint/>
      },
      {
        path: "faction/:factionId",
        element: <Faction/>
      }
    ]
  }
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router}/>
  </React.StrictMode>
);
