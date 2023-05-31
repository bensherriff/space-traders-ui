import {NavLink} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { SessionStorage, Text } from '../js';
import { ProgressBarWithLabel } from '../components/ProgressBar';

export default function Fleet() {
  const [ships, setShips] = useState({});

  useEffect(() => {
    list_ships();
  }, []);

  async function list_ships() {
    invoke("list_ships", { token: SessionStorage.getSessionToken() }).then((response) => {
      setShips(response.data);
    });
  }

  return (
    <div>
      {ships && Array.isArray(ships)? (
        ships.map((ship, index) => (
          <Ship key={index} ship={ship}/>
        ))
      ): <></>}
      <button className="float-right" onClick={list_ships}>Refresh</button>
    </div>
  )
}

function Ship({ship}) {
  return (
    <NavLink to={`/fleet/${ship.symbol}`}>
      <div className='my-4 p-2 border-stone-900 border-2 text-l shadow-2xl bg-stone-700 hover:bg-stone-800'>
        <h1 className='font-bold'>{ship.registration.name} <i>{Text.capitalize(ship.registration.role)} {ship.frame.name.split(" ")[1]}</i></h1>
        <div className='flex justify-between'>
        <ProgressBarWithLabel label="Condition" text={`${ship.frame.condition}/100`} percentage={ship.frame.condition} />
        <ProgressBarWithLabel label="Fuel" text={`${ship.fuel.current}/${ship.fuel.capacity}`} percentage={ship.fuel.current/ship.fuel.capacity*100} />
        </div>
      </div>
    </NavLink>
  )
}