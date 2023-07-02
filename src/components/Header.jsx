import { Text, State } from "../js";
import { NavLink } from "react-router-dom";
import { useRecoilState} from "recoil";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faHouse, faCoins } from '@fortawesome/free-solid-svg-icons'

export default function Header() {
  const [agent, setAgent] = useRecoilState(State.agentState);

  let split = agent.headquarters.split("-");
  let system = `${split[0]}-${split[1]}`;

  return (
    <header className="bg-stone-900 sticky top-0">
      <div className="flex justify-between px-2 py-1 ">
          <span className="flex">
            <HeaderItem>{agent.symbol}</HeaderItem>
            <HeaderItem to={`/faction/${agent.startingFaction}`}>({agent.startingFaction})</HeaderItem>
          </span>
          <span>
            <HeaderItem className="pr-10"><>
            <FontAwesomeIcon className="text-yellow-300 pr-2" icon={faCoins}/>{Text.currency(agent.credits)}
            </></HeaderItem>
            <HeaderItem to={`/system/${system}/${agent.headquarters}`}><>
              <FontAwesomeIcon className="text-orange-600 pr-2" icon={faHouse}/>{agent.headquarters}
            </></HeaderItem>
            <HeaderItem to={`/system/${system}`}>({system})</HeaderItem>
          </span>
      </div>
    </header>
  )
}

function HeaderItem({ children, to, className="" }) {
  if (to) {
    return (
      <NavLink className={`${className} inline-block px-1 text-gray-200 select-none text-xl`} to={to}>{children}</NavLink>
    )  
  }
  return (
    <h1 className={`${className} inline-block px-1 text-gray-200 select-none text-xl`}>{children}</h1>
  )
}