import { useEffect, useState } from 'react';
import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { Storage } from '../js';

export default function Ship() {
  const {shipId} = useParams();
  const [ship, setShip] = useState(null);
  const [waypoint, setWaypoint] = useState(null);
  const [market, setMarket] = useState(null);
  const [shipyard, setShipyard] = useState(null);

  useEffect(() => {
    get_ship();
  }, []);

  async function get_ship() {
    invoke("get_ship", { token: Storage.getSessionToken(), symbol: shipId}).then((response) => {

    });
  }

  async function get_waypoint() {

  }

  async function get_market() {

  }

  async function get_shipyard() {

  }

  return (
    <div>
      {shipId}
      <h1>Waypoint</h1>
    </div>
  )
}