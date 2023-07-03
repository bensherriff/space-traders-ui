import { Outlet } from "react-router-dom";
import { useRecoilValue} from "recoil";
import "./Root.css";
import Login from "./components/Login";
import { Storage, State } from "./js";
import Header from "./components/Header";
import Footer from "./components/Footer";
import { IAgent } from "./js/interfaces";

export default function Root() {
  const agent: IAgent | null = useRecoilValue(State.agentState);

  return (
    <>
      {!agent? (
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
