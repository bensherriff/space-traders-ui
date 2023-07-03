export function SystemObject({ x, y, r, fill, onMouseEnter=() => {}, onMouseLeave=() => {}, onClick=() => {} } : { x: number, y: number, r: number, fill: string, onMouseEnter?: any, onMouseLeave?: any, onClick?: any }) {
  return (
    <circle
      className="cursor-pointer"
      cx={x}
      cy={y}
      r={r}
      fill={fill}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      onClick={onClick}
    />
  )
}