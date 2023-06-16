import { Outlet } from "react-router-dom";
import { useRecoilState} from "recoil";
import "./Root.css";
import Login from "./components/Login";
import { Storage, State } from "./js";
import Header from "./components/Header";
import Footer from "./components/Footer";

export default function Root() {
  const [agent, setAgent] = useRecoilState(State.agentState);

  return (
    <>
      {!agent || !agent.symbol? (
        <Login />
      ):
        <div className="min-h-screen flex flex-col">
          <Header/>
          <div className="mx-6 my-2">
            <Outlet />
          </div>
          <Footer />
        </div>
      }
    </>
  );
}
