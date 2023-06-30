import { useEffect, useState } from "react"
import { useNavigate } from "react-router-dom";
import { Text } from "../js";

//TODO https://cloudfour.com/thinks/generating-svg-solar-systems-part-1-setting-the-scene/
export default function SystemSVG({system}) {
  const [star, setStar] = useState(<></>);
  const [planets, setPlanets] = useState(<></>);
  const [displayText, setDisplayText] = useState("");
  const navigate = useNavigate();

  const width = 1000;
  const height = 1000;
  const scale = 6;

  useEffect(() => {
    draw();
  }, []);

  function draw() {
    setStar(drawStar(50));
    let _planets = [];
    for (let i in system.waypoints) {
      if ([
        "PLANET", "GAS_GIANT", "JUMP_GATE", "ASTEROID_FIELD", "NEBULA", "DEBRIS_FIELD", "GRAVITY_WELL"
      ].includes(system.waypoints[i].type)) {
        _planets.push(drawPlanet(20, system.waypoints[i]));
      }
    }
    setPlanets(_planets);
  }

  function drawStar(size) {
    return (
      <circle 
        cx={width/2}
        cy={height/2}
        r={size}
        fill={"#f7d916"}
        onMouseEnter={() => {setDisplayText(Text.capitalize(system.type))}}
        onMouseLeave={() => {setDisplayText("")}}
      />
    )
  }

  function drawPlanet(size, waypoint) {
    const color = Text.waypointTypeColor(waypoint.type);
    const split = waypoint.symbol.split("-");
    const x = width/2 + (waypoint.x * scale);
    const y = height/2 + (waypoint.y * scale);

    return (
      <circle
        cx={x}
        cy={y}
        r={size}
        fill={color.bg}
        onClick={() => {
          navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
        }}
        onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
        onMouseLeave={() => {setDisplayText("")}}
      />
    )
  }

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlnsXlink="http://www.w3.org/1999/xlink"
      viewBox="0 0 1000 1000"
      style={{background: "#000"}}
      role="img"
      aria-labelledby="solarSystemTitle"
      aria-describedby="solarSystemDescription"
    >
      <desc id="solarSystemDescription">
        
      </desc>

      <g className="js-svg-wrapper">
        {star}
        {planets}
        <text x={50} y={height - 10} font-size="2em" fill={"#fff"}>{displayText}</text>
      </g>

      <style>

      </style>
    </svg>
  )
}