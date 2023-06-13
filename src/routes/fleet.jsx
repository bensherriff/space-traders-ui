import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage } from '../js';
import { ShipInfo } from "../components/Ship";
import { Button } from "../components";

export default function Fleet() {
  const [ships, setShips] = useState({});

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
    <div>
      {ships && Array.isArray(ships)? (
        ships.map((ship, index) => (
          <ShipInfo key={index} ship={ship}/>
        ))
      ): <></>}
      <Button className="float-right" onClick={list_ships}>Refresh</Button>
    </div>
  )
}