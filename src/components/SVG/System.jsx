import { useEffect, useRef, useState } from "react"
import { useNavigate } from "react-router-dom";
import { Text } from "../../js";
import { v4 as uuidv4 } from 'uuid';
import { SystemObject } from ".";
import { atom } from "recoil";
import { useRecoilState, useRecoilValue } from "recoil";

const WIDTH = 1000;
const HEIGHT = 1000;
const MIN_ZOOM = 1;
const MAX_ZOOM = 20;
const SCROLL_SENSITIVITY = 0.005;

const mapState = atom({
  key: 'mapState',
  default: {
    cameraZoom: 6,
    lastZoom: 6,
    cameraOffset: {x: 0, y: 0}
  }
})

export default function SystemMap({ system }) {
  const svgRef = useRef(null);
  const [displayText, setDisplayText] = useState("");
  const [map, setMap] = useRecoilState(mapState);

  const waypointTypes = ["PLANET", "GAS_GIANT", "JUMP_GATE", "ASTEROID_FIELD", "NEBULA", "DEBRIS_FIELD", "GRAVITY_WELL"];
  const orbitalTypes = ["MOON", "ORBITAL_STATION"];

  const waypoints = system.waypoints.filter((waypoint) => waypointTypes.includes(waypoint.type));
  const orbitals = system.waypoints.filter((waypoint) => orbitalTypes.includes(waypoint.type));

  const satelliteOrbitRadius = 5 * map.cameraZoom;

  useEffect(() => {
    if (svgRef.current) {
      const svg = svgRef.current;

      let isDragging = false;
      let dragStart = {x: 0, y: 0};

      function adjustZoom(zoomAmount, zoomFactor) {
        if (!isDragging) {
          let cameraZoom = map.cameraZoom;
          if (zoomAmount) {
            cameraZoom += zoomAmount;
          } else if (zoomFactor) {
            zoomFactor*lastZoom;
          }
          cameraZoom = Math.min(cameraZoom, MAX_ZOOM);
          cameraZoom = Math.max(cameraZoom, MIN_ZOOM);
          setMap({
            ...map,
            cameraZoom: cameraZoom
          });
        }
      }

      function getEventLocation(e) {
        if (e.touches && e.touches.length == 1) {
          return { x: e.touches[0].clientX, y: e.touches[0].clientY }
        } else if (e.clientX && e.clientY) {
          return { x: e.clientX, y: e.clientY }
        }
      }

      function onPointerDown(event) {
        isDragging = true;
        dragStart.x = getEventLocation(event).x / map.cameraZoom - map.cameraOffset.x;
        dragStart.y = getEventLocation(event).y / map.cameraZoom - map.cameraOffset.y;
      }

      function onPointerUp(event) {
        isDragging = false;
        lastZoom = map.cameraZoom;
      }

      function onPointerMove(event) {
        if (isDragging) {
          let offset = {x: 0, y: 0};
          offset.x = getEventLocation(event).x / map.cameraZoom - dragStart.x;
          // offset.x = Math.min(offset.x, WIDTH*2);
          // offset.x = Math.max(offset.x, -WIDTH/2);
          offset.y = getEventLocation(event).y / map.cameraZoom - dragStart.y;
          // offset.y = Math.min(offset.y, HEIGHT*2);
          // offset.y = Math.max(offset.y, -HEIGHT/2);
          setMap({
            ...map,
            cameraOffset: offset
          });
        }
      }

      svg.addEventListener("wheel", (event) => adjustZoom(-event.deltaY*(SCROLL_SENSITIVITY*(map.cameraZoom/3))));
      svg.addEventListener("mousedown", onPointerDown);
      svg.addEventListener("mouseup", onPointerUp);
      svg.addEventListener("mousemove", onPointerMove);

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
          <WaypointOrbit x={waypoint.x} y={waypoint.y}/>
        ))}
        {orbitals.map((waypoint) => (
          <SatelliteOrbit x={waypoint.x} y={waypoint.y} orbitRadius={satelliteOrbitRadius}/>
        ))}
        <Star system={system} setDisplayText={setDisplayText}/>
        {waypoints.map((waypoint) => (
          <Waypoint waypoint={waypoint} setDisplayText={setDisplayText}/>
        ))}
        {orbitals.map((waypoint) => (
          <Satellite waypoint={waypoint} setDisplayText={setDisplayText} orbitRadius={satelliteOrbitRadius}/>
        ))}
        <text x={50} y={HEIGHT - 10} fontSize="2em" fill={"#fff"}>{displayText}</text>
      </g>
    </svg>
  )
}

function Star({ system, setDisplayText=() => {} }) {
  let map = useRecoilValue(mapState);
  const color = Text.systemTypeColor(system.type);
  return (
    <SystemObject
      x={map.cameraOffset.x + WIDTH/2}
      y={map.cameraOffset.y + HEIGHT/2}
      r={7 * map.cameraZoom}
      fill={color.bg}
      onMouseEnter={() => {setDisplayText(Text.capitalize(system.type))}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function Waypoint({ waypoint, setDisplayText=() => {} }) {
  let map = useRecoilValue(mapState);
  const navigate = useNavigate();
  const color = Text.waypointTypeColor(waypoint.type);
  const split = waypoint.symbol.split("-");
  const x = WIDTH/2 + (waypoint.x * map.cameraZoom);
  const y = HEIGHT/2 + (waypoint.y * map.cameraZoom);

  return (
    <SystemObject
      key={waypoint.symbol}
      x={map.cameraOffset.x + x}
      y={map.cameraOffset.y +y}
      r={waypoint.type == "JUMP_GATE"? 2 * map.cameraZoom: 3 * map.cameraZoom}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function WaypointOrbit({ x=WIDTH/2, y=HEIGHT/2 }) {
  let map = useRecoilValue(mapState);
  const orbitRadius = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2)) * map.cameraZoom;

  return (
    <circle
      key={uuidv4()}
      cx={map.cameraOffset.x + WIDTH/2}
      cy={map.cameraOffset.y + HEIGHT/2}
      r={orbitRadius}
      stroke={"#fff"}
      strokeDasharray={"5,5"}
      fillOpacity={0}
    />
  )
}

function Satellite({ waypoint, setDisplayText=() => {}, orbitRadius=5 }) {
  let map = useRecoilValue(mapState);
  const navigate = useNavigate();
  const color = Text.waypointTypeColor(waypoint.type);
  const split = waypoint.symbol.split("-");
  const planetX = WIDTH/2 + (waypoint.x * map.cameraZoom);
  const planetY = HEIGHT/2 + (waypoint.y * map.cameraZoom);
  const x = planetX + orbitRadius * Math.cos((2 * Math.PI));
  const y = planetY + orbitRadius * Math.sin((2 * Math.PI));

  return (
    <SystemObject
      key={waypoint.symbol}
      x={map.cameraOffset.x + x}
      y={map.cameraOffset.y + y}
      r={1 * map.cameraZoom}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function SatelliteOrbit({ x=WIDTH/2, y=HEIGHT/2, orbitRadius=5 }) {
  let map = useRecoilValue(mapState);
  const planetX = WIDTH/2 + (x * map.cameraZoom);
  const planetY = HEIGHT/2 + (y * map.cameraZoom);

  return (
    <circle
      key={uuidv4()}
      cx={map.cameraOffset.x + planetX}
      cy={map.cameraOffset.y + planetY}
      r={orbitRadius}
      stroke={"#fff"}
      stroke-dasharray={"5,5"}
      fillOpacity={0}
    />
  )
}