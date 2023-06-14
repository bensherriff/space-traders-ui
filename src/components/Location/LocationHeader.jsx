import Tag from '../../components/Tag';
import { Text } from '../../js';

export default function LocationHeader({symbol, type}) {
  let bgColor = "bg-green-700";
  let textColor = "text-white";
  if (type == 'NEUTRON_STAR') {
    textColor = "text-neutron-star-text"
    bgColor = "bg-neutron-star-bg"
  } else if (type == 'RED_STAR') {
    textColor = "text-red-star-text"
    bgColor = "bg-red-star-bg"
  } else if (type == 'ORANGE_STAR') {
    textColor = "text-orange-star-text"
    bgColor = "bg-orange-star-bg"
  } else if (type == 'BLUE_STAR') {
    textColor = "text-blue-star-text"
    bgColor = "bg-blue-star-bg"
  } else if (type == 'YOUNG_STAR') {
    textColor = "text-young-star-text"
    bgColor = "bg-young-star-bg"
  } else if (type == 'WHITE_DWARF') {
    textColor = "text-white-dwarf-text"
    bgColor = "bg-white-dwarf-bg"
  } else if (type == 'BLACK_HOLE') {
    textColor = "text-black-hole-text"
    bgColor = "bg-black-hole-bg"
  } else if (type == 'HYPERGIANT') {
    textColor = "text-hypergiant-text"
    bgColor = "bg-hypergiant-bg"
  } else if (type == 'NEBULA') {
    textColor = "text-nebula-text"
    bgColor = "bg-nebula-bg"
  } else if (type == 'UNSTABLE') {
    textColor = "text-unstable-text"
    bgColor = "bg-unstable-bg"
  } else if (type == 'PLANET') {
    textColor = "text-planet-text"
    bgColor = "bg-planet-bg"
  } else if (type == 'GAS_GIANT') {
    textColor = "text-gas-giant-text"
    bgColor = "bg-gas-giant-bg"
  } else if (type == 'MOON') {
    textColor = "text-moon-text"
    bgColor = "bg-moon-bg"
  } else if (type == 'ORBITAL_STATION') {
    textColor = "text-orbital-station-text"
    bgColor = "bg-orbital-station-bg"
  } else if (type == 'JUMP_GATE') {
    textColor = "text-jump-gate-text"
    bgColor = "bg-jump-gate-bg"
  } else if (type == 'ASTEROID_FIELD') {
    textColor = "text-asteroid-field-text"
    bgColor = "bg-asteroid-field-bg"
  // } else if (type == 'NEBULA') {
  //   textColor = "text--text"
  //   bgColor = "bg--bg"
  } else if (type == 'DEBRIS_FIELD') {
    textColor = "text-debris-field-text"
    bgColor = "bg-debris-field-bg"
  } else if (type == 'GRAVITY_WELL') {
    textColor = "text-gravity-well-text"
    bgColor = "bg-gravity-well-bg"
  }
  return (
    <span className='flex'>
      <Tag text={Text.capitalize(type)} bgColor={bgColor} textColor={textColor}/>
      <h1 className='cursor-default select-none text-left text-4xl'>{symbol}</h1>
    </span>
  )
}