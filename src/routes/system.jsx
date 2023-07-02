import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage, Text } from '../js';
import { NavLink } from "react-router-dom";
import { SystemHeader } from '../components/Location/LocationHeader';
import SystemMap  from '../components/SVG/System';

export default function System() {
  const {systemId} = useParams();
  const [system, setSystem] = useState(null);
  const [waypoints, setWaypoints] = useState([]);
  const [ships, setShips] = useState([]);
  const [markets, setMarkets] = useState([]);

  useEffect(() => {
    get_system();
    get_waypoints();
    get_ships();
  }, []);

  useEffect(() => {
    get_markets();
  }, [waypoints]);

  async function get_system() {
    invoke("get_system", { token: Storage.getToken(), system: systemId }).then((response) => {
      if (response && response.data) {
        setSystem(response.data);
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  async function get_waypoints() {
    invoke("get_waypoints", { token: Storage.getToken(), system: systemId }).then((response) => {
      if (response && response.data) {
        setWaypoints(response.data);
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  async function get_ships() {
    invoke("list_ships", { token: Storage.getToken(), system: systemId }).then((response) => {
      if (response && response.data) {
        setShips(response.data);
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  async function get_markets() {
    let _markets = [];
    for (let waypoint of waypoints) {
      let hasMarket = waypoint.traits.some(trait => trait.symbol === 'MARKETPLACE');
      if (hasMarket) {
        invoke("get_market", { token: Storage.getToken(), system: systemId, waypoint: waypoint.symbol }).then((response) => {
          if (response && response.data) {
            _markets.push(response.data);
            setMarkets(_markets);
          } else if (response && response.error) {
            console.error(response.error);
          }
        });
      }
    }
  }

  return (
    <div>
      {system? (
        <div className='select-none cursor-default'>
          <SystemHeader symbol={system.symbol} type={system.type}/>
          <div className='flex ml-6'>
            <span>Sector {system.sectorSymbol}</span>
            <span className='pl-4'>({system.x},{system.y})</span>
          </div>
          <hr className='mb-5'/>
          <div className='flex justify-between'>
            <div className='w-1/2'>
              {waypoints.map((waypoint, index) => {
                let colors = Text.waypointTypeColor(waypoint.type);
                let hasMarket = waypoint.traits.some(trait => trait.symbol === 'MARKETPLACE');
                let hasShipyard = waypoint.traits.some(trait => trait.symbol === 'SHIPYARD');
                // This might have some innacuracies, since waypointSymbol is the current location, unless the ship is in transit.
                let shipsPresent = ships.filter(ship => ship.nav.waypointSymbol === waypoint.symbol);
                return (
                  <NavLink key={index} to={`/system/${systemId}/${waypoint.symbol}`}>
                    <div className={`block pr-2 mb-1 mr-1 rounded-md bg-stone-900 hover:bg-stone-950 text-lg`}>
                      <span className={`${colors.bgTW} ${colors.textTW} inline-flex w-48 p-3 rounded-md`}>{Text.capitalize(waypoint.type)}</span>
                      <span className='pl-2 mr-2'>{waypoint.symbol}</span>
                      {hasMarket? <span className='ml-1 p-1 hover:text-black text-black rounded-xl bg-green-500'>MP</span>: <></>}
                      {hasShipyard? <span className='ml-1 p-1 hover:text-black text-black rounded-xl bg-cyan-500'>SY</span>: <></>}
                      {shipsPresent.length > 0? <span className='ml-1 p-1 hover:text-black text-black rounded-xl bg-yellow-500'>{shipsPresent.length}</span>: <></>}
                    </div>
                  </NavLink>
                )
              })}
            </div>
            <div className='w-1/2'>
              <SystemMap system={system}/>
            </div>
          </div>
        </div>
      ): <></>}
    </div>
  )
}