export function SystemObject({ x=WIDTH/2, y=HEIGHT/2, r=1, fill="", onMouseEnter=() => {}, onMouseLeave=() => {}, onClick=() => {} }) {
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