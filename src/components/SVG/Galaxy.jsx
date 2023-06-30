import { useEffect, useRef, useState } from "react"
import { useNavigate } from "react-router-dom";
import { Text } from "../../js";
import { v4 as uuidv4 } from 'uuid';

const WIDTH = 1000;
const HEIGHT = 1000;
const MIN_ZOOM = 1;
const MAX_ZOOM = 20;
const SCROLL_SENSITIVITY = 0.005;

export default function Galaxy({ system }) {
  const svgRef = useRef(null);
  const [displayText, setDisplayText] = useState("");
  const [scale, setScale] = useState(6);

  const waypointTypes = ["PLANET", "GAS_GIANT", "JUMP_GATE", "ASTEROID_FIELD", "NEBULA", "DEBRIS_FIELD", "GRAVITY_WELL"];
  const orbitalTypes = ["MOON", "ORBITAL_STATION"];

  const waypoints = system.waypoints.filter((waypoint) => waypointTypes.includes(waypoint.type));
  const orbitals = system.waypoints.filter((waypoint) => orbitalTypes.includes(waypoint.type));

  const satelliteOrbitRadius = 5 * scale;

  useEffect(() => {
    if (svgRef.current) {
      const svg = svgRef.current;

      let point = svg.createSVGPoint();

      let isDragging = false;
      let cameraZoom = 6;

      function adjustZoom(zoomAmount, zoomFactor) {
        if (!isDragging) {
          if (zoomAmount) {
            cameraZoom += zoomAmount;
          } else if (zoomFactor) {
            zoomFactor*lastZoom;
          }
          cameraZoom = Math.min(cameraZoom, MAX_ZOOM);
          cameraZoom = Math.max(cameraZoom, MIN_ZOOM);
          setScale(cameraZoom);
        }
      }

      //TODO work on cursor positon for dragging
      function cursorPosition(event) {
        point.x = event.clientX;
        point.y = event.clientY;
        return point.matrixTransform(svg.getScreenCTM().inverse());
      }

      svg.addEventListener("wheel", (event) => adjustZoom(-event.deltaY*SCROLL_SENSITIVITY));
      svg.addEventListener("mousemove", (event) => {
        let loc = cursorPosition(event);
      });
      document.getElementById( "solar-system" ).onwheel = function(event){
        event.preventDefault();
      };
      
      document.getElementById( "solar-system" ).onmousewheel = function(event){
          event.preventDefault();
      };
    }

  }, []);

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlnsXlink="http://www.w3.org/1999/xlink"
      viewBox="0 0 1000 1000"
      style={{background: "#000"}}
      role="img"
      aria-labelledby="solarSystemTitle"
      aria-describedby="solarSystemDescription"
      ref={svgRef}
      id="solar-system"
    >
      <g className="js-svg-wrapper">
        {waypoints.map((waypoint) => (
          <WaypointOrbit x={waypoint.x} y={waypoint.y} scale={scale} />
        ))}
        {orbitals.map((waypoint) => (
          <SatelliteOrbit x={waypoint.x} y={waypoint.y} scale={scale} orbitRadius={satelliteOrbitRadius} />
        ))}
        <Star system={system} setDisplayText={setDisplayText} scale={scale} />
        {waypoints.map((waypoint) => (
          <Waypoint waypoint={waypoint} setDisplayText={setDisplayText} scale={scale} />
        ))}
        {orbitals.map((waypoint) => (
          <Satellite waypoint={waypoint} setDisplayText={setDisplayText} scale={scale} orbitRadius={satelliteOrbitRadius} />
        ))}
        <text x={50} y={HEIGHT - 10} fontSize="2em" fill={"#fff"}>{displayText}</text>
      </g>
    </svg>
  )
}

function SystemObject({ x=WIDTH/2, y=HEIGHT/2, r=1, fill="", onMouseEnter=() => {}, onMouseLeave=() => {}, onClick=() => {} }) {
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

function Star({ system, setDisplayText=() => {}, scale=1 }) {
  const color = Text.systemTypeColor(system.type);
  return (
    <SystemObject
      x={WIDTH/2}
      y={HEIGHT/2}
      r={7 * scale}
      fill={color.bg}
      onMouseEnter={() => {setDisplayText(Text.capitalize(system.type))}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function Waypoint({ waypoint, setDisplayText=() => {}, scale=1 }) {
  const navigate = useNavigate();
  const color = Text.waypointTypeColor(waypoint.type);
  const split = waypoint.symbol.split("-");
  const x = WIDTH/2 + (waypoint.x * scale);
  const y = HEIGHT/2 + (waypoint.y * scale);

  return (
    <SystemObject
      key={waypoint.symbol}
      x={x}
      y={y}
      r={waypoint.type == "JUMP_GATE"? 2 * scale: 3 * scale}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function WaypointOrbit({ x=WIDTH/2, y=HEIGHT/2, scale=1 }) {
  const orbitRadius = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2)) * scale;

  return (
    <circle
      key={uuidv4()}
      cx={WIDTH/2}
      cy={HEIGHT/2}
      r={orbitRadius}
      stroke={"#fff"}
      strokeDasharray={"5,5"}
      fillOpacity={0}
    />
  )
}

function Satellite({ waypoint, setDisplayText=() => {}, orbitRadius=5, scale=1 }) {
  const navigate = useNavigate();
  const color = Text.waypointTypeColor(waypoint.type);
  const split = waypoint.symbol.split("-");
  const planetX = WIDTH/2 + (waypoint.x * scale);
  const planetY = HEIGHT/2 + (waypoint.y * scale);
  const x = planetX + orbitRadius * Math.cos((2 * Math.PI));
  const y = planetY + orbitRadius * Math.sin((2 * Math.PI));

  return (
    <SystemObject
      key={waypoint.symbol}
      x={x}
      y={y}
      r={1 * scale}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function SatelliteOrbit({ x=WIDTH/2, y=HEIGHT/2, orbitRadius=5, scale=1 }) {
  const planetX = WIDTH/2 + (x * scale);
  const planetY = HEIGHT/2 + (y * scale);

  return (
    <circle
      key={uuidv4()}
      cx={planetX}
      cy={planetY}
      r={orbitRadius}
      stroke={"#fff"}
      stroke-dasharray={"5,5"}
      fillOpacity={0}
    />
  )
}