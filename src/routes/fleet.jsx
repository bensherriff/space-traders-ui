import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage } from '../js';
import { ShipInfo } from "../components/Ship";

export default function Fleet() {
  const [ships, setShips] = useState({});

  useEffect(() => {
    list_ships();
  }, []);

  async function list_ships() {
    invoke("list_ships", { token: Storage.getToken() }).then((response) => {
      setShips(response.data);
      response.data.forEach(ship => {
        Storage.setShip(ship.symbol, ship);
      });
    });
  }

  return (
    <div>
      {ships && Array.isArray(ships)? (
        ships.map((ship, index) => (
          <ShipInfo key={index} ship={ship}/>
        ))
      ): <></>}
      <button className="float-right" onClick={list_ships}>Refresh</button>
    </div>
  )
}