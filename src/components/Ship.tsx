import {NavLink, useNavigate} from 'react-router-dom';
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Storage, Text } from '../js';
import { ProgressBarWithLabel, ProgressBar } from './ProgressBar';
import { Button, CountdownTimer } from '.';
import { IResponse, IShip, ISystem, ISystemWaypoint } from '../js/interfaces';

export function NavStatus({ship, setShip=() => {}, link = false}: {ship: IShip, setShip?: Function, link?: boolean}) {
  const timeout = (new Date(ship.nav.route.arrival).getTime()/1000 - Date.now()/1000);

  useEffect(() => {
    getShipNav();
  }, []);

  async function getShipNav() {
    if (timeout <= 0) {
      let response: IResponse = await invoke('get_ship_nav', { token: Storage.getToken(), symbol: ship.symbol });
      if (response && response.data) {
        setShip({
          ...ship,
          nav: response.data
        });
      }
    }
  }

  if (ship.nav.status === "IN_TRANSIT") {
    return (
      <span>
        In transit to {link ? <NavLink to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}>{ship.nav.waypointSymbol}</NavLink> : ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.destination.type)})
        <br />
        <span className='text-sky-500'><CountdownTimer duration={timeout}/></span>
      </span>
    )
  } else if (ship.nav.status === "IN_ORBIT") {
    return (
      <span>
        Orbiting {link ? <NavLink to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}>{ship.nav.waypointSymbol}</NavLink> : ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.destination.type)})
      </span>
    )
  } else if (ship.nav.status === "DOCKED") {
    return (
      <span>
        Docked at {link ? <NavLink to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}>{ship.nav.waypointSymbol}</NavLink> : ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.destination.type)})
      </span>
    )
  }
}

export function CargoInfo({ship}: {ship: IShip}) {
  return (
    <div className='my-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
      <h1 className='text-lg mr-4'>Cargo</h1>
      <ProgressBar text={`${ship.cargo.units}/${ship.cargo.capacity}`} percentage={ship.cargo.units/ship.cargo.capacity*100} />
      <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
        <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
          <tr>
            <th scope='col' className='px-6 py-3'>Name</th>
            <th scope='col' className='px-6 py-3'>Units</th>
            <th scope='col' className='px-6 py-3'>Description</th>
          </tr>
        </thead>
        <tbody>
          {ship.cargo.inventory.map((item, index) => (
            <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
              <th scope='row' className='px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(item.name)}</th>
              <td className='px-6 py-4'>{item.units}</td>
              <td className='px-6 py-4'>{item.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

export function ModulesInfo({ship}: {ship: IShip}) {
  return (
    <div className='w-full mr-0.5 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
      <h1 className='text-lg mr-4'>Modules</h1>
      <ProgressBar text={`${ship.modules.length}/${ship.frame.moduleSlots}`} percentage={ship.modules.length/ship.frame.moduleSlots*100} />
      <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
        <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
          <tr>
            <th scope='col' className='px-6 py-3'>Name</th>
            <th scope='col' className='px-6 py-3'>Capacity</th>
            <th scope='col' className='px-6 py-3'>Range</th>
            <th scope='col' className='px-6 py-3'>Description</th>
          </tr>
        </thead>
        <tbody>
          {ship.modules.map((module, index) => (
            <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
              <th scope='row' className='px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(module.name)}</th>
              <td className='px-6 py-4'>{module.capacity}</td>
              <td className='px-6 py-4'>{module.range}</td>
              <td className='px-6 py-4'>{module.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

export function MountsInfo({ship}: {ship: IShip}) {
  return (
    <div className='w-full ml-0.5 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
      <h1 className='text-lg mr-4'>Mounts</h1>
      <ProgressBar text={`${ship.mounts.length}/${ship.frame.mountingPoints}`} percentage={ship.mounts.length/ship.frame.mountingPoints*100} />
      <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
        <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
          <tr>
            <th scope='col' className='px-6 py-3'>Name</th>
            <th scope='col' className='px-6 py-3'>Strength</th>
            <th scope='col' className='px-6 py-3'>Deposits</th>
            <th scope='col' className='px-6 py-3'>Description</th>
          </tr>
        </thead>
        <tbody>
          {ship.mounts.map((mount, index) => (
            <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
              <th scope='row' className='px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(mount.name)}</th>
              <td className='px-6 py-4'>{mount.strength}</td>
              <td className='px-6 py-4'>
                {mount.deposits? (
                  <>
                    {mount.deposits.map((deposit, index) => (
                      <span key={index}>{Text.capitalize(deposit)} </span>
                    ))}
                  </>
                ): <></>}
              </td>
              <td className='px-6 py-4'>{mount.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

export function Navigation({ship, setShip}: {ship: IShip, setShip: Function}) {
  const [system, setSystem] = useState<ISystem | null>(null);
  const [waypoint, setWaypoint] = useState<ISystemWaypoint | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    if (ship.nav.status !== "IN_TRANSIT") {
      get_system();
      get_waypoint();
    }
  }, []);

  async function orbit_ship() {
    let response: IResponse = await invoke("orbit_ship", { token: Storage.getToken(), symbol: ship.symbol});
    if (response && response.data) {
      setShip({
        ...ship,
        nav: response.data.nav
      });
    }
  }

  async function dock_ship() {
    let response: IResponse = await invoke("dock_ship", { token: Storage.getToken(), symbol: ship.symbol});
    if (response && response.data) {
      setShip({
        ...ship,
        nav: response.data.nav
      });
    }
  }

  async function get_system() {
    let response: IResponse = await invoke("get_system", { token: Storage.getToken(), system: ship.nav.systemSymbol });
    if (response && response.data) {
      setSystem(response.data);
    }
  }

  async function get_waypoint() {
    let response: IResponse = await invoke("get_waypoint", { token: Storage.getToken(), system: ship.nav.systemSymbol, waypoint: ship.nav.waypointSymbol });
    if (response && response.data) {
      setWaypoint(response.data);
    }
  }

  async function navigate_ship(e: any) {
    e.preventDefault();
    const destination = e.target['select-system'].value;
    let response: IResponse = await invoke("navigate_ship", { token: Storage.getToken(), symbol: ship.symbol, waypoint: destination, system: ship.nav.systemSymbol});
    if (response && response.data) {
      setShip({
        ...ship,
        fuel: {
          ...ship.fuel,
          current: response.data.fuel.current
        },
        nav: response.data.nav
      });
      const timeout = (new Date(ship.nav.route.arrival).getTime() - Math.min(new Date(ship.nav.route.departureTime).getTime(), Date.now()));
      setTimeout(async () => {
        let response: IResponse = await invoke("get_ship_nav", { token: Storage.getToken(), symbol: ship.symbol });
        if (response && response.data) {
          setShip({
            ...ship,
            nav: response.data
          });
        }
      }, timeout);
    }
  }

  return (
    <div className='my-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
      <h1 className='text-lg mr-4'>Navigation</h1>
      <NavStatus ship={ship} setShip={setShip} link={true}/>
      {ship.nav.status === "IN_ORBIT"? (
        <Button onClick={dock_ship}>Dock</Button>
      ): ship.nav.status === "DOCKED"? (
        <Button onClick={orbit_ship}>Orbit</Button>
      ): <></>}
      {system && system.waypoints && ship.nav.status !== "IN_TRANSIT" && ship.nav.status !== "DOCKED"? (
        <>
          <form method='post' onSubmit={navigate_ship}>
            <select className='text-black bg-stone-200' name='select-system' id='select-system'>
              {system.waypoints.map((waypoint, index) => {
                if (waypoint.symbol === ship.nav.waypointSymbol) {
                  return (<option key={`select-system_${index}`} value={waypoint.symbol} disabled>{Text.capitalize(waypoint.type)} {waypoint.symbol}</option>)
                } else {
                  return (<option key={`select-system_${index}`} value={waypoint.symbol}>{Text.capitalize(waypoint.type)} {waypoint.symbol}</option>)
                }
              })}
            </select>
            <Button type='submit'>Navigate</Button>
          </form>
        </>
      ): <></>}
    </div>
  )
}