import { State } from "../js";
import { NavLink } from "react-router-dom";
import { useRecoilState} from "recoil";

export default function Footer() {
  const [agent, setAgent] = useRecoilState(State.agentState);

  function logout() {
    setAgent({});
  }

  return (
    <footer className="bg-stone-950 mt-auto sticky bottom-0 text-center">
      <div className="flex justify-between">
        <NavLink className={`block w-full h-full mr-0.5 py-5 text-gray-200 bg-stone-900 select-none text-xl`} to={`/fleet`}>Fleet</NavLink>
        <NavLink className={`block w-full h-full mr-0.5 py-5 text-gray-200 bg-stone-900 select-none text-xl`} to={`/contracts`}>Contracts</NavLink>
        <NavLink className={`block w-full h-full mr-0.5 py-5 text-gray-200 bg-stone-900 select-none text-xl`} to={`/galaxy`}>Galaxy</NavLink>
        <NavLink onClick={logout} className={`block w-full h-full py-5 text-gray-200 bg-stone-900 select-none text-xl`} to={`/`}>Logout</NavLink>
      </div>
    </footer>
  )
}