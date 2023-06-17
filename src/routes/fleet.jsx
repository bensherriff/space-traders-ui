import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage, Text } from '../js';
import { NavStatus } from "../components/Ship";
import { ProgressBarWithLabel } from "../components/ProgressBar";
import { NavLink } from "react-router-dom";

export default function Fleet() {
  const [ships, setShips] = useState(null);

  useEffect(() => {
    list_ships();
  }, []);

  async function list_ships() {
    invoke("list_ships", { token: Storage.getToken() }).then((response) => {
      if (response && response.data) {
        setShips(response.data);
      }
    });
  }

  return (
    <ul role="list" className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
      {ships? (ships.map((ship) => (
        <li key={ship.registration.name} className="col-span-1 divide-y divide-gray-200 rounded-lg bg-stone-600 shadow">
          <div className="flex w-full items-center justify-between space-x-6 p-6">
            <div className="flex-1 truncate">
              <div className="flex items-center space-x-3">
                <h3 className="truncate text-lg font-medium text-gray-900">{ship.symbol}</h3>
                <span className="inline-flex flex-shrink-0 items-center rounded-full bg-green-50 px-1.5 py-0.5 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">
                  {Text.capitalize(ship.registration.role)} {ship.frame.name.split(" ")[1]}
                </span>
              </div>
              <p className="mt-1 truncate text-sm text-gray-300"><NavStatus ship={ship}/></p>
              <ProgressBarWithLabel label="Condition" text={`${ship.frame.condition}/100`} percentage={ship.frame.condition} />
              <ProgressBarWithLabel label="Fuel" text={`${ship.fuel.current}/${ship.fuel.capacity}`} percentage={ship.fuel.current/ship.fuel.capacity*100} />
            </div>
          </div>
          <div>
            <div className="-mt-px flex divide-x divide-gray-200">
              <div className="flex w-0 flex-1">
                <NavLink
                  to={`/fleet/${ship.symbol}`}
                  className="relative -mr-px inline-flex w-0 flex-1 items-center justify-center gap-x-3 rounded-bl-lg border border-transparent py-4 text-sm font-semibold text-gray-900"
                >
                  Ship Info
                </NavLink>
              </div>
              <div className="-ml-px flex w-0 flex-1">
                <NavLink
                  to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}
                  className="relative inline-flex w-0 flex-1 items-center justify-center gap-x-3 rounded-br-lg border border-transparent py-4 text-sm font-semibold text-gray-900"
                >
                  Waypoint
                </NavLink>
              </div>
            </div>
          </div>
        </li>
      ))): <></>}
    </ul>
  )
}