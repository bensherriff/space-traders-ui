import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage, Text } from '../js';
import { SystemMap } from '../components/Canvas';
import { NavLink } from "react-router-dom";
import LocationHeader from '../components/Location/LocationHeader';

export default function System() {
  const {systemId} = useParams();
  const [system, setSystem] = useState({});

  useEffect(() => {
    get_system();
  }, [systemId]);

  async function get_system() {
    invoke("get_system", { token: Storage.getToken(), system: systemId }).then((response) => {
      if (response && response.data) {
        setSystem(response.data);
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  return (
    <div>
      {system && system.type? (
        <div>
          <LocationHeader symbol={system.symbol} type={system.type}/>
          <hr className='mb-5'/>
          <ul>
            <li>Sector: {system.sectorSymbol}</li>
            <li>Coordinates: ({system.x},{system.y})</li>
          </ul>
          {system.waypoints && Array.isArray(system.waypoints)? (
            <div className='block p-4 shadow-md rounded-lg border bg-stone-800 border-stone-900'>
              <h2 className='text-2xl text-center'>System Map</h2>
              <hr/>
              <div className='flex justify-between m-2'>
                <ul className='block'>
                  {system.waypoints.map((waypoint, index) => (
                    <NavLink key={index} to={`/system/${systemId}/${waypoint.symbol}`}>
                      <li className='block p-2 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none'>
                        <span className='mr-4'>{Text.capitalize(waypoint.type)}</span>
                        <span className='float-right'>{waypoint.symbol}</span>
                      </li>
                    </NavLink>
                  ))}
                </ul>
                <div className='block ml-4 border-2 border-stone-500'>
                  <SystemMap system={system}/>
                </div>
              </div>
            </div>
          ): <></>}
        </div>
      ): <></>}
    </div>
  )
}