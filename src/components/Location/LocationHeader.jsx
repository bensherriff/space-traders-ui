import Tag from '../../components/Tag';
import { Text } from '../../js';

export function SystemHeader({symbol, type}) {
  let colors = Text.systemTypeColor(type);
  return (
    <span className='flex m-1 text-left'>
      <Tag text={Text.capitalize(type)} bgColor={colors.bg} textColor={colors.text}/>
      <h1 className='pl-1 cursor-default select-none text-4xl'>{symbol}</h1>
    </span>
  )
}

export function WaypointHeader({symbol, type}) {
  let colors = Text.waypointTypeColor(type);
  return (
    <span className='flex m-1 text-left'>
      <Tag text={Text.capitalize(type)} bgColor={colors.bg} textColor={colors.text}/>
      <h1 className='pl-1 cursor-default select-none text-4xl'>{symbol}</h1>
    </span>
  )
}