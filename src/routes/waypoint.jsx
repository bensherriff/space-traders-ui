import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { SessionStorage, Text } from '../js';
import Tag from '../components/Tag';
import { NavLink } from "react-router-dom";

export default function Waypoint() {
  const {systemId, waypointId} = useParams();
  const [waypoint, setWaypoint] = useState({});
  const [waypointTraits, setWaypointTraits] = useState([]);

  useEffect(() => {
    get_waypoint();
  }, [systemId, waypointId]);

  async function get_waypoint() {
    invoke("get_waypoint", { token: SessionStorage.getSessionToken(), system: systemId, waypoint: waypointId}).then((response) => {
      setWaypoint(response.data);
      setWaypointTraits(response.data.traits);
      console.log(response);
    })
  }

  return (
    <div>
      {waypoint && waypoint.type? (
        <div>
          <h1 className='text-center text-2xl'>{waypoint.symbol} ({Text.capitalize(waypoint.type)})</h1>
          <hr className='mb-5'/>
          <div className='mb-5 text-center'>
            {waypointTraits && Array.isArray(waypointTraits)? (
              waypointTraits.map((trait, index) => (
                <Tag key={index} text={trait.name}/>
              ))
            ): <></>}
          </div>
          <ul>
            <li>System: <NavLink to={`/system/${systemId}`}>{systemId}</NavLink></li>
            {waypoint.faction? 
            <li>Faction: <NavLink to={`/faction/${waypoint.faction.symbol}`}>
              {waypoint.faction.symbol}</NavLink></li>
            : <></>}
            <li>Coordinates: ({waypoint.x},{waypoint.y})</li>
          </ul>
          {waypoint.orbitals && Array.isArray(waypoint.orbitals)? (
            <div>
              <hr className='my-2'/>
              <h2 className='text-xl'>Orbitals</h2>
              <ul>
                {waypoint.orbitals.map((orbital, index) => (
                  <li key={index} className='pl-4'><NavLink to={`/system/${systemId}/${orbital.symbol}`}>{orbital.symbol}</NavLink></li>
                ))}
              </ul>
            </div> 
          ): <></>}
          {waypointTraits.includes('Marketplace')? (
            <>test</>
          ): <></>}
        </div>
      ): <></>}
    </div>
  )
}