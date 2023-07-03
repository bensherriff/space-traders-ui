import Tag from '../Tag';
import { Text } from '../../js';
import { NavLink } from "react-router-dom";

export function SystemHeader({ symbol, type }: { symbol: string, type: string }) {
  let colors = Text.systemTypeColor(type);
  return (
    <span className='flex m-1 text-left'>
      <Tag text={Text.capitalize(type)} bgColor={colors.bgTW} textColor={colors.textTW}/>
      <h1 className='pl-1 cursor-default select-none text-4xl'>{symbol}</h1>
    </span>
  )
}

export function WaypointHeader({ waypoint }: { waypoint: any }) {
  let colors = Text.waypointTypeColor(waypoint.type);
  return (
    <span className='flex m-1 text-left'>
      <Tag text={Text.capitalize(waypoint.type)} bgColor={colors.bgTW} textColor={colors.textTW}/>
      <h1 className='pl-1 cursor-default select-none text-4xl'>{waypoint.symbol}</h1>
      {waypoint.faction? (
        <NavLink to={`/faction/${waypoint.faction.symbol}`}><h2 className='select-none text-2xl pl-2 pt-2'>({waypoint.faction.symbol})</h2></NavLink>
      ):<></>}
    </span>
  )
}