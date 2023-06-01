import {NavLink} from 'react-router-dom';
import { Text } from '../../js';
import { ProgressBarWithLabel, ProgressBar } from '../ProgressBar';

export function ShipInfo({ship}) {
  return (
    <NavLink to={`/fleet/${ship.symbol}`}>
      <div className='my-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 hover:bg-stone-800 rounded-xl'>
        <span className='flex'>
          <h1 className='font-bold mr-4'>{ship.registration.name} <i>{Text.capitalize(ship.registration.role)} {ship.frame.name.split(" ")[1]}</i></h1>
          <NavStatus ship={ship}/>
        </span>
        <div className='flex justify-between'>
          <ProgressBarWithLabel label="Condition" text={`${ship.frame.condition}/100`} percentage={ship.frame.condition} />
          <ProgressBarWithLabel label="Fuel" text={`${ship.fuel.current}/${ship.fuel.capacity}`} percentage={ship.fuel.current/ship.fuel.capacity*100} />
        </div>
      </div>
    </NavLink>
  )
}

export function NavStatusLink({ship}) {
  if (ship.nav.status === "IN_TRANSIT") {
    return (
      <span>
        In transit to <NavLink to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}>
          {ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.destination.type)})
          </NavLink>
      </span>
    )
  } else if (ship.nav.status === "IN_ORBIT") {
    return (
      <span>
        Orbiting <NavLink to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}>
          {ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.departure.type)})
        </NavLink>
      </span>
    )
  } else if (ship.nav.status === "DOCKED") {
    return (
      <span>
        Docked at <NavLink to={`/system/${ship.nav.systemSymbol}/${ship.nav.waypointSymbol}`}>
          {ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.departure.type)})
        </NavLink>
      </span>
    )
  }
}

export function NavStatus({ship}) {
  if (ship.nav.status === "IN_TRANSIT") {
    return (
      <span>In transit to {ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.destination.type)})</span>
    )
  } else if (ship.nav.status === "IN_ORBIT") {
    return (
      <span>Orbiting {ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.departure.type)})</span>
    )
  } else if (ship.nav.status === "DOCKED") {
    return (
      <span>Docked at {ship.nav.waypointSymbol} ({Text.capitalize(ship.nav.route.departure.type)})</span>
    )
  }
}

export function CargoInfo({ship}) {
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

export function ModulesInfo({ship}) {
  return (
    <div className='m-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
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

export function MountsInfo({ship}) {
  return (
    <div className='m-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
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

export function Navigation({ship}) {
  return (
    <div className='my-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
      <h1 className='text-lg mr-4'>Navigation</h1>
    </div>
  )
}