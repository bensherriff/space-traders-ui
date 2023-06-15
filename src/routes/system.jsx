import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage, Text } from '../js';
import { SystemMap } from '../components/Canvas';
import { NavLink } from "react-router-dom";
import { SystemHeader } from '../components/Location/LocationHeader';
import Tag from '../components/Tag';
import SystemSVG from '../components/SystemSVG';

export default function System() {
  const {systemId} = useParams();
  const [system, setSystem] = useState(null);

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
              {system.waypoints? (
                <>
                  {system.waypoints.map((waypoint, index) => {
                    let colors = Text.waypointTypeColor(waypoint.type);
                    console.log(colors);
                    return (
                      <NavLink key={index} to={`/system/${systemId}/${waypoint.symbol}`}>
                        <div className={`block px-2 py-8 mb-1 mr-1 rounded-md bg-stone-900 hover:bg-stone-950 text-lg`}>
                          <span className={`${colors.bgTW} ${colors.textTW} p-3 rounded-md`}>{Text.capitalize(waypoint.type)}</span>
                          <span className='float-right'>{waypoint.symbol}</span>
                        </div>
                      </NavLink>
                    )
                  })}
                </>
              ): <></>}
            </div>
            <div className='w-1/2'>
              <SystemSVG system={system}/>
            </div>
          </div>
        </div>
      ): <></>}
    </div>
  )
}