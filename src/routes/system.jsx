import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { SessionStorage, Text } from '../js';
import { SystemMap } from '../components/Canvas';
import { NavLink } from "react-router-dom";

export default function System() {
  const {systemId} = useParams();
  const [system, setSystem] = useState({});

  useEffect(() => {
    get_system();
  }, [systemId]);

  async function get_system() {
    invoke("get_system", { token: SessionStorage.getSessionToken(), system: systemId }).then((response) => {
      setSystem(response.data);
    })
  }

  return (
    <div>
      {system && system.type? (
        <div>
          <h1 className='text-center text-2xl'>{system.symbol} ({Text.capitalize(system.type)})</h1>
          <hr className='mb-5'/>
          <ul>
            <li>Sector: {system.sectorSymbol}</li>
            <li>Coordinates: ({system.x},{system.y})</li>
          </ul>
          {system.waypoints && Array.isArray(system.waypoints)? (
            <div className=''>
              <hr className='my-2'/>
              <h2 className='text-xl'>System Map</h2>
              <div className='flex justify-center m-2'>
                <SystemMap system={system}/>
                <ul className='block mx-4'>
                  {system.waypoints.map((waypoint, index) => (
                    <li key={index} className=''><NavLink to={`/system/${systemId}/${waypoint.symbol}`}>{Text.capitalize(waypoint.type)} {waypoint.symbol} ({waypoint.x},{waypoint.y})</NavLink></li>
                  ))}
                </ul>
              </div>
            </div>
          ): <></>}
        </div>
      ): <></>}
    </div>
  )
}