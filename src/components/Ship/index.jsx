import {NavLink} from 'react-router-dom';
import { Text } from '../../js';
import { ProgressBarWithLabel } from '../ProgressBar';

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