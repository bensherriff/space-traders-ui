import { useEffect, useRef, useState } from "react"
import { useNavigate } from "react-router-dom";
import { Text } from "../../js";
import { v4 as uuidv4 } from 'uuid';
import { SystemObject } from ".";

const WIDTH = 1000;
const HEIGHT = 1000;
const MIN_ZOOM = 2;
const MAX_ZOOM = 20;
const SCROLL_SENSITIVITY = 0.005;
const HIDE_SATELLITES_ZOOM = 2.8;

export default function SystemMap({ system }) {
  const svgRef = useRef(null);
  const [displayText, setDisplayText] = useState("");
  const [cameraZoom, setCameraZoom] = useState(6);
  const [cameraOffset, setCameraOffset] = useState({ x: 0, y: 0 });

  const waypointTypes = ["PLANET", "GAS_GIANT", "JUMP_GATE", "ASTEROID_FIELD", "NEBULA", "DEBRIS_FIELD", "GRAVITY_WELL"];
  const orbitalTypes = ["MOON", "ORBITAL_STATION"];

  const waypoints = system.waypoints.filter((waypoint) => waypointTypes.includes(waypoint.type));
  const orbitals = system.waypoints.filter((waypoint) => orbitalTypes.includes(waypoint.type));

  const satelliteOrbitRadius = 5 * cameraZoom;

  useEffect(() => {
    if (svgRef.current) {
      const svg = svgRef.current;

      let isDragging = false;
      let dragStart = {x: 0, y: 0};
      let lastZoom = 6;
      let zoom = 6;
      let offset = {x: 0, y: 0};

      function adjustZoom(zoomAmount, zoomFactor) {
        if (!isDragging) {
          if (zoomAmount) {
            zoom += zoomAmount;
          } else if (zoomFactor) {
            zoom = zoomFactor*lastZoom;
          }
          zoom = Math.min(zoom, MAX_ZOOM);
          zoom = Math.max(zoom, MIN_ZOOM);
          setCameraZoom(zoom);
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
        dragStart.x = getEventLocation(event).x / zoom - offset.x;
        dragStart.y = getEventLocation(event).y / zoom - offset.y;
      }

      function onPointerUp(event) {
        isDragging = false;
        lastZoom = zoom;
      }

      function onPointerMove(event) {
        if (isDragging) {
          offset = {
            x: getEventLocation(event).x / zoom - dragStart.x,
            y: getEventLocation(event).y / zoom - dragStart.y
          }
          setCameraOffset(offset);
        }
      }

      svg.addEventListener("wheel", (event) => adjustZoom(-event.deltaY*(SCROLL_SENSITIVITY*(zoom/3))));
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

  let previousOrbitalCoords = { x: 0, y: 0 };
  let orbitalCount = 1;

  function calculateOrbitalRadius(waypoint, index) {
    if (waypoint.x == previousOrbitalCoords.x && waypoint.y == previousOrbitalCoords.y) {
      orbitalCount++;
    } else {
      orbitalCount = 1;
    }
    // Reset the previous orbital coords if we're on the last orbital, otherwise set it to the current waypoint
    previousOrbitalCoords = index == orbitals.length - 1? {x: 0, y: 0}: { x: waypoint.x, y: waypoint.y };
    return satelliteOrbitRadius * (orbitalCount * 0.6) + (2 * cameraZoom);
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
      ref={svgRef}
      id="solar-system"
    >
      <g className="js-svg-wrapper">
        {waypoints.map((waypoint) => (
          <WaypointOrbit key={`waypoint-orbit-${waypoint.symbol}`} x={waypoint.x} y={waypoint.y} cameraZoom={cameraZoom} cameraOffset={cameraOffset}/>
        ))}
        {cameraZoom > HIDE_SATELLITES_ZOOM? 
        <>
          {orbitals.map((waypoint, index) => {
            let orbitRadius = calculateOrbitalRadius(waypoint, index);
            return (
              <SatelliteOrbit key={`satellite-orbit-${waypoint.symbol}`} x={waypoint.x} y={waypoint.y} orbitRadius={orbitRadius} cameraZoom={cameraZoom} cameraOffset={cameraOffset}/>
            );
          })}
        </>: <></>}
        <Star system={system} setDisplayText={setDisplayText} cameraZoom={cameraZoom} cameraOffset={cameraOffset}/>
        {waypoints.map((waypoint) => (
          <Waypoint key={`waypoint-${waypoint.symbol}`} waypoint={waypoint} setDisplayText={setDisplayText} cameraZoom={cameraZoom} cameraOffset={cameraOffset}/>
        ))}
        {cameraZoom > HIDE_SATELLITES_ZOOM?
        <>
          {orbitals.map((waypoint, index) => {
            let orbitRadius = calculateOrbitalRadius(waypoint, index);
            return (
              <Satellite key={`satellite-${waypoint.symbol}`} waypoint={waypoint} setDisplayText={setDisplayText} orbitRadius={orbitRadius} cameraZoom={cameraZoom} cameraOffset={cameraOffset}/>
            )
          })}
        </>: <></>}
        <text x={50} y={HEIGHT - 10} fontSize="2em" fill={"#fff"}>{displayText}</text>
      </g>
    </svg>
  )
}

function Star({ system, setDisplayText=() => {}, cameraZoom, cameraOffset }) {
  const color = Text.systemTypeColor(system.type);
  return (
    <SystemObject
      x={cameraOffset.x + WIDTH/2}
      y={cameraOffset.y + HEIGHT/2}
      r={7 * cameraZoom}
      fill={color.bg}
      onMouseEnter={() => {setDisplayText(Text.capitalize(system.type))}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function Waypoint({ waypoint, setDisplayText=() => {}, cameraZoom, cameraOffset }) {
  const navigate = useNavigate();
  const color = Text.waypointTypeColor(waypoint.type);
  const split = waypoint.symbol.split("-");
  const x = WIDTH/2 + (waypoint.x * cameraZoom);
  const y = HEIGHT/2 + (waypoint.y * cameraZoom);

  return (
    <SystemObject
      key={waypoint.symbol}
      x={cameraOffset.x + x}
      y={cameraOffset.y +y}
      r={waypoint.type == "JUMP_GATE"? 2 * cameraZoom: 3 * cameraZoom}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function WaypointOrbit({ x=WIDTH/2, y=HEIGHT/2, cameraZoom, cameraOffset }) {
  const orbitRadius = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2)) * cameraZoom;

  return (
    <circle
      key={uuidv4()}
      cx={cameraOffset.x + WIDTH/2}
      cy={cameraOffset.y + HEIGHT/2}
      r={orbitRadius}
      stroke={"#fff"}
      strokeDasharray={"5,10"}
      fillOpacity={0}
    />
  )
}

function Satellite({ waypoint, setDisplayText=() => {}, orbitRadius=5, cameraZoom, cameraOffset }) {
  const navigate = useNavigate();
  const color = Text.waypointTypeColor(waypoint.type);
  const split = waypoint.symbol.split("-");
  const planetX = WIDTH/2 + (waypoint.x * cameraZoom);
  const planetY = HEIGHT/2 + (waypoint.y * cameraZoom);
  const x = planetX + orbitRadius * Math.cos((2 * Math.PI));
  const y = planetY + orbitRadius * Math.sin((2 * Math.PI));

  return (
    <SystemObject
      key={waypoint.symbol}
      x={cameraOffset.x + x}
      y={cameraOffset.y + y}
      r={1 * cameraZoom}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${waypoint.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(waypoint.type)} ${waypoint.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}

function SatelliteOrbit({ x=WIDTH/2, y=HEIGHT/2, orbitRadius=5, cameraZoom, cameraOffset }) {
  const planetX = WIDTH/2 + (x * cameraZoom);
  const planetY = HEIGHT/2 + (y * cameraZoom);

  return (
    <circle
      key={uuidv4()}
      cx={cameraOffset.x + planetX}
      cy={cameraOffset.y + planetY}
      r={orbitRadius}
      stroke={"#fff"}
      strokeDasharray={"5,20"}
      fillOpacity={0}
    />
  )
}