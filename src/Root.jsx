import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import "./Root.css";
import Login from "./components/Login";
import { Storage } from "./js";
import Header from "./components/Header";

export default function Root() {
  const [agent, setAgent] = useState({});

  useEffect(() => {
    var _agent = Storage.getSessionAgent();
    setAgent(_agent);
  }, [])

  if (!agent.symbol) {
    return <Login setAgent={setAgent} />
  }

  return (
    <>
      <Header/>
      <div className="mx-6">
        <Outlet />
      </div>
    </>
  );
}
