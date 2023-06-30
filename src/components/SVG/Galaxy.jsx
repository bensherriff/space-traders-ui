import { useEffect, useRef, useState } from "react"
import { useNavigate } from "react-router-dom";
import { Text } from "../../js";
import { SystemObject } from ".";

const WIDTH = 120000;
const HEIGHT = 120000;
const MIN_ZOOM = 1;
const MAX_ZOOM = 20;
const SCROLL_SENSITIVITY = 0.005;

export default function GalaxyMap({ systems }) {
  const svgRef = useRef(null);
  const [displayText, setDisplayText] = useState("");
  const [cameraZoom, setCameraZoom] = useState(6);
  const [cameraOffset, setCameraOffset] = useState({x: WIDTH/2, y: HEIGHT/2});
  const [lastZoom, setLastZoom] = useState(6);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({x: 0, y: 0});

  useEffect(() => {
    if (svgRef.current) {
      const svg = svgRef.current;
      const point = svg.createSVGPoint();

      function adjustZoom(zoomAmount, zoomFactor) {
        let zoom = cameraZoom;
        if (!isDragging) {
          if (zoomAmount) {
            zoom += zoomAmount;
          } else if (zoomFactor) {
            zoomFactor*lastZoom;
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
        setIsDragging(true);
        setDragStart({
          x: getEventLocation(event).x / cameraZoom - cameraOffset.x,
          y: getEventLocation(event).y / cameraZoom - cameraOffset.y
        });
      }

      function onPointerUp(event) {
        setIsDragging(false);
        setLastZoom(cameraZoom);
      }

      function onPointerMove(event) {
        let {x, y} = cursorPosition(event);
        let cursor = { x: x * cameraZoom + WIDTH/2, y: y * cameraZoom + HEIGHT/2 };
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
      document.getElementById( "galaxy" ).onwheel = function(event){
        event.preventDefault();
      };
      
      document.getElementById( "galaxy" ).onmousewheel = function(event){
          event.preventDefault();
      };
    }

  }, []);

  let systemObjects = [];
  for (let i = 0; i < 10; i++) {
    systemObjects.push(<System key={systems[i].symbol} system={systems[i]} setDisplayText={setDisplayText} scale={cameraZoom} />);
  }

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlnsXlink="http://www.w3.org/1999/xlink"
      viewBox="0 0 1000 1000"
      style={{background: "#000"}}
      role="img"
      aria-labelledby="galaxyTitle"
      aria-describedby="galaxyDescription"
      ref={svgRef}
      id="galaxy"
    >
      <g className="js-svg-wrapper">
        {systemObjects}
        <text x={50} y={HEIGHT - 10} fontSize="2em" fill={"#fff"}>{displayText}</text>
      </g>
    </svg>
  )
}

function System({ system, setDisplayText=() => {}, scale=1 }) {
  const navigate = useNavigate();
  const color = Text.systemTypeColor(system.type);
  const split = system.symbol.split("-");
  const x = WIDTH/2 + (system.x * scale);
  const y = HEIGHT/2 + (system.y * scale);

  console.log(system, x, y);

  return (
    <SystemObject
      key={system.symbol}
      x={x}
      y={y}
      r={100 * scale}
      fill={color.bg}
      onClick={() => {
        navigate(`/system/${split[0]}-${split[1]}/${system.symbol}`);
      }}
      onMouseEnter={() => {setDisplayText(`${Text.capitalize(system.type)} ${system.symbol}`)}}
      onMouseLeave={() => {setDisplayText("")}}
    />
  )
}
