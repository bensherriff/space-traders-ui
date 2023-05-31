import { useEffect, useState } from "react";
import { Storage, Text } from "../../js";
import { NavLink } from "react-router-dom";

export default function Header() {
  const [agent, setAgent] = useState(Storage.getAgent());

  useEffect(() => {
    setAgent(Storage.getAgent());
  }, [])

  let split = agent.headquarters.split("-");
  let system = `${split[0]}-${split[1]}`;

  return (
    <header className="h-14 border-b border-slate-900 mb-2 bg-sky-900 shadow-md">
      <ul className="flex justify-between px-4">
        <HeaderItem text={agent.symbol} to={"/fleet"} />
        <HeaderItem text={Text.currency(agent.credits)}/>
        <div className="flex">
          <HeaderItem text={agent.headquarters} to={`/system/${system}/${agent.headquarters}`} />
          <HeaderItem text={`(${system})`} to={`/system/${system}`} />
        </div>
        <HeaderItem text={agent.startingFaction} to={`/faction/${agent.startingFaction}`} />
      </ul>
    </header>
  )
}

function HeaderItem({ text, to }) {
  if (to) {
    return (
      <li className="">
        <NavLink className="inline-block px-2 py-3 text-gray-200 select-none text-xl" to={to}>{text}</NavLink>
      </li>
    )  
  }
  return (
    <li className="">
      <h1 className="inline-block px-2 py-3 text-gray-200 select-none text-xl">{text}</h1>
    </li>
  )
}