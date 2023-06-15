import Tag from '../../components/Tag';
import { Text } from '../../js';
import { NavLink } from "react-router-dom";

export function SystemHeader({symbol, type}) {
  let colors = Text.systemTypeColor(type);
  return (
    <span className='flex m-1 text-left'>
      <Tag text={Text.capitalize(type)} bgColor={colors.bgTW} textColor={colors.textTW}/>
      <h1 className='pl-1 cursor-default select-none text-4xl'>{symbol}</h1>
    </span>
  )
}

export function WaypointHeader({symbol, type, faction}) {
  let colors = Text.waypointTypeColor(type);
  return (
    <span className='flex m-1 text-left'>
      <Tag text={Text.capitalize(type)} bgColor={colors.bgTW} textColor={colors.textTW}/>
      <h1 className='pl-1 cursor-default select-none text-4xl'>{symbol}</h1>
      <NavLink to={`/faction/${faction}`}><h2 className='select-none text-2xl pl-2 pt-2'>({faction})</h2></NavLink>
    </span>
  )
}