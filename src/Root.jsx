import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import "./Root.css";
import Login from "./components/Login";
import { Storage } from "./js";
import Header from "./components/Header";
import Footer from "./components/Footer";

export default function Root() {
  const [agent, setAgent] = useState({});

  useEffect(() => {
    var _agent = Storage.getAgent();
    setAgent(_agent);
  }, []);

  return (
    <>
      {!agent || !agent.symbol? (
        <Login setAgent={setAgent} />
      ):
        <div className="min-h-screen flex flex-col">
          <Header/>
          <div className="mx-6 my-2">
            <Outlet />
          </div>
          <Footer setAgent={setAgent} />
        </div>
      }
    </>
  );
}
