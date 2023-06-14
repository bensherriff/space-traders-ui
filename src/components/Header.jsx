import { useEffect, useState } from "react";
import { Storage, Text } from "../js";
import { NavLink } from "react-router-dom";

export default function Header() {
  const [agent, setAgent] = useState(Storage.getAgent());

  useEffect(() => {
    setInterval(() => {
      setAgent(Storage.getAgent());
    }, 60000);
  }, [])

  let split = agent.headquarters.split("-");
  let system = `${split[0]}-${split[1]}`;

  return (
    <header className="border-b border-slate-900 mb-2 bg-slate-500 shadow-sm">
      <ul className="flex justify-between px-4">
        <div className="flex">
        <HeaderItem to={"/fleet"}>{agent.symbol}</HeaderItem>
        <HeaderItem to={`/faction/${agent.startingFaction}`}>({agent.startingFaction})</HeaderItem>
        </div>
        <div className="flex">
          <HeaderItem>{Text.currency(agent.credits)}</HeaderItem>
          <HeaderItem>|</HeaderItem>
          <HeaderItem to={`/system/${system}/${agent.headquarters}`}>{agent.headquarters}</HeaderItem>
          <HeaderItem to={`/system/${system}`}>{system}</HeaderItem>
        </div>
      </ul>
    </header>
  )
}

function HeaderItem({ to, children }) {
  if (to) {
    return (
      <li className="">
        <NavLink className="inline-block px-1 py-1 text-gray-200 select-none text-xl" to={to}>{children}</NavLink>
      </li>
    )  
  }
  return (
    <li className="">
      <h1 className="inline-block px-1 py-1 text-gray-200 select-none text-xl">{children}</h1>
    </li>
  )
}