import { State } from "../js";
import { NavLink } from "react-router-dom";
import { useRecoilState} from "recoil";

export default function Footer() {
  const [agent, setAgent] = useRecoilState(State.agentState);

  function logout() {
    setAgent(null);
  }

  return (
    <footer className="bg-stone-950 mt-auto sticky bottom-0 text-center">
      <div className="flex justify-between">
        <NavLink
          className={`block w-full h-full py-5 text-gray-200 bg-stone-900 select-none text-xl hover:bg-stone-950 border border-stone-950 border-b-0 border-l-0`}
          to={`/fleet`}
        >
          Fleet
        </NavLink>
        <NavLink
          className={`block w-full h-full py-5 text-gray-200 bg-stone-900 select-none text-xl hover:bg-stone-950 border border-stone-950 border-b-0`}
          to={`/contracts`}
        >
          Contracts
        </NavLink>
        <NavLink
          className={`block w-full h-full py-5 text-gray-200 bg-stone-900 select-none text-xl hover:bg-stone-950 border border-stone-950 border-b-0`}
          to={`/galaxy`}
        >
          Galaxy
        </NavLink>
        <NavLink 
          onClick={logout}
          className={`block w-full h-full py-5 text-gray-200 bg-stone-900 select-none text-xl hover:bg-stone-950 border border-stone-950 border-b-0 border-r-0`}
          to={`/`}
        >
          Logout
        </NavLink>
      </div>
    </footer>
  )
}