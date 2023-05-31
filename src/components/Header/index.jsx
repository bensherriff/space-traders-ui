import { useEffect, useState } from "react";
import { SessionStorage } from "../../js";
import { NavLink } from "react-router-dom";

export default function Header() {
  const [agent, setAgent] = useState(SessionStorage.getSessionAgent());

  useEffect(() => {
    setAgent(SessionStorage.getSessionAgent());
  }, [])

  let split = agent.headquarters.split("-");
  let system = `${split[0]}-${split[1]}`;

  return (
    <header className="border-b mb-2 bg-blue-900">
      <ul className="flex justify-between px-4">
        <HeaderItem text={agent.symbol} to={"/fleet"} />
        <HeaderItem text={`${agent.credits} ${'\u2124'}`}/>
        <HeaderItem text={agent.headquarters} to={`/system/${system}/${agent.headquarters}`} />
        <HeaderItem text={agent.startingFaction} to={`/faction/${agent.startingFaction}`} />
      </ul>
    </header>
  )
}

function HeaderItem({ text, to }) {
  if (to) {
    return (
      <li className="mr-3">
        <NavLink className="inline-block px-4 py-4 text-gray-200 select-none text-xl" to={to}>{text}</NavLink>
      </li>
    )  
  }
  return (
    <li className="mr-3">
      <h1 className="inline-block px-4 py-4 text-gray-200 select-none text-xl">{text}</h1>
    </li>
  )
}