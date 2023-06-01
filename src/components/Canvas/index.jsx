import { useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";
import { Text } from "../../js";

const SCALE = 10;
const MAX_ZOOM = 2;
const MIN_ZOOM = 0.5;
const SCROLL_SENSITIVITY = 0.0005;

class SolarObject {
  constructor(context, x, y, radius, color, hide, symbol, type) {
    this.context = context;
    this.x = x;
    this.y = y;
    this.radius = radius;
    this.color = color;
    this.hide = hide;
    this.symbol = symbol;
    this.type = type;
  }

  draw() {
    this.context.beginPath();
    this.context.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
    this.context.fillStyle = this.color;
    this.context.fill();
    this.context.closePath();
  }

  update() {
    this.draw();
  }
}

export function SystemMap({system}) {
  const canvasRef = useRef(null);
  const navigate = useNavigate();

  useEffect(() => {
    if (canvasRef.current) {
      const canvas = canvasRef.current;
      const context = canvas.getContext('2d');

      let width = 500;
      let height = 500;
      let cameraOffset = { x: width/2, y: height/2};
      let cameraZoom = MIN_ZOOM;
      const solarObjects = [];

      let isDragging = false;
      let dragStart = { x: 0, y: 0 };
      let textLocation = { x: 0, y: 0 };
      let cursorText = "";
      let lastZoom = cameraZoom;

      const centerX = width/2;
      const centerY = height/2;

      let sun = new SolarObject(context, centerX, centerY, 30, "yellow", false, system.symbol, system.type);
      solarObjects.push(sun);
      system.waypoints.forEach((waypoint) => {
        const x = centerX + (waypoint.x * SCALE);
        const y = centerY + (waypoint.y * SCALE);
        let object = null;
        if (waypoint.type == "PLANET") {
          object = new SolarObject(context, x, y, 10, "#4363d8", false, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "GAS_GIANT") {
          object = new SolarObject(context, x, y, 10, "#3cb44b", false, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "MOON") {
          object = new SolarObject(context, x, y, 10, "#f58231", true, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "ORBITAL_STATION") {
          object = new SolarObject(context, x, y, 10, "#ffffff", true, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "JUMP_GATE") {
          object = new SolarObject(context, x, y, 10, "#aaffc3", false, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "ASTEROID_FIELD") {
          object = new SolarObject(context, x, y, 10, "#9A6324", false, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "NEBULA") {
          object = new SolarObject(context, x, y, 10, "#469990", false, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "DEBRIS_FIELD") {
          object = new SolarObject(context, x, y, 10, "#808000", false, waypoint.symbol, waypoint.type);
        } else if (waypoint.type == "GRAVITY_WELL") {
          object = new SolarObject(context, x, y, 10, "#000000", false, waypoint.symbol, waypoint.type);
        } else {
          object = new SolarObject(context, x, y, 10, "#800000", false, waypoint.symbol, waypoint.type);
        }
        solarObjects.push(object);
      });

      function draw() {
        canvas.width = width;
        canvas.height = height;

        // context.translate(width/2, height/2);
        context.scale(cameraZoom, cameraZoom);
        context.translate(-width/2 + cameraOffset.x, -height/2 + cameraOffset.y);
        context.clearRect(0, 0, width, height);

        solarObjects.forEach(object => {
          if (!object.hide) {
            object.draw();
          }
        });

        drawText(`${cursorText}`, textLocation.x, textLocation.y);
        
        requestAnimationFrame(draw);
      }

      function drawText(text, x, y, size = 18 / cameraZoom, font = "courier", lineHeight = 15 / cameraZoom) {
        let lines = text.split('\n');
        lines.forEach((line, index) => {
          context.font = `${size}px ${font}`;
          context.fillStyle = "#f032e6"
          context.fillText(line, x, y + (index * lineHeight));
        });
      }

      function getEventLocation(e) {
        if (e.touches && e.touches.length == 1) {
          return { x: e.touches[0].clientX, y: e.touches[0].clientY }
        } else if (e.clientX && e.clientY) {
          return { x: e.clientX, y: e.clientY }
        }
      }

      function onPointerDown(e) {
        isDragging = true;
        dragStart.x = getEventLocation(e).x / cameraZoom - cameraOffset.x;
        dragStart.y = getEventLocation(e).y / cameraZoom - cameraOffset.y;
      }

      function onPointerUp(e) {
        isDragging = false;
        lastZoom = cameraZoom;
      }

      function onPointerMove(e) {
        let {x, y} = getCursorPosition(e);
        x = x;
        y = y;
        let cursor = { x: x * SCALE + 250, y: y * SCALE + 250 };
        textLocation.x = cursor.x + (10 / cameraZoom);
        textLocation.y = cursor.y + (30 / cameraZoom);
        cursorText = '';
        solarObjects.map((object) => {
          if ((cursor.x <= object.x + object.radius && cursor.x >= object.x - object.radius) &&
              (cursor.y <= object.y + object.radius && cursor.y >= object.y - object.radius)
          ) {
            if (cursorText) {
              cursorText += `  ${Text.capitalize(object.type)}\n  ${object.symbol}\n`;
            } else {
              cursorText += `${Text.capitalize(object.type)}\n${object.symbol}\n`;
            }
          }
        })
        if (isDragging) {
          cameraOffset.x = getEventLocation(e).x / cameraZoom - dragStart.x;
          cameraOffset.x = Math.min(cameraOffset.x, width*2);
          cameraOffset.x = Math.max(cameraOffset.x, -width/2);
          cameraOffset.y = getEventLocation(e).y / cameraZoom - dragStart.y;
          cameraOffset.y = Math.min(cameraOffset.y, height*2);
          cameraOffset.y = Math.max(cameraOffset.y, -height/2);
        }
      }

      function onPointerClick(e) {
        let {x, y} = getCursorPosition(e);
        x = x;
        y = y;
        let cursor = { x: x * SCALE + 250, y: y * SCALE + 250 };
        solarObjects.map((object) => {
          if ((cursor.x <= object.x + object.radius && cursor.x >= object.x - object.radius) &&
              (cursor.y <= object.y + object.radius && cursor.y >= object.y - object.radius) &&
              !object.hide
          ) {
            const split = object.symbol.split("-");
            if (split.length == 3) {
              navigate(`/system/${split[0]}-${split[1]}/${object.symbol}`)
            } else if (split.length == 2) {
              navigate(`/system/${split[0]}-${split[1]}`)
            }
          }
        })
      }

      function getCursorPosition(event) {
        const rect = canvas.getBoundingClientRect()
        const x = ((event.clientX - rect.left) / cameraZoom - cameraOffset.x) / SCALE;
        const y = ((event.clientY - rect.top) / cameraZoom - cameraOffset.y) / SCALE;
        return {x, y}
      }

      function adjustZoom(zoomAmount, zoomFactor) {
        if (!isDragging) {
          if (zoomAmount) {
            cameraZoom += zoomAmount;
          } else if (zoomFactor) {
            cameraZoom = zoomFactor*lastZoom;
          }
          cameraZoom = Math.min(cameraZoom, MAX_ZOOM);
          cameraZoom = Math.max(cameraZoom, MIN_ZOOM);
        }
      }

      canvas.addEventListener('mousedown', onPointerDown);
      canvas.addEventListener('mouseup', onPointerUp);
      canvas.addEventListener('mousemove', onPointerMove);
      canvas.addEventListener('click', onPointerClick);
      canvas.addEventListener('wheel', (e) => adjustZoom(-e.deltaY*SCROLL_SENSITIVITY));

      document.getElementById( "system-canvas" ).onwheel = function(event){
        event.preventDefault();
      };
      
      document.getElementById( "system-canvas" ).onmousewheel = function(event){
          event.preventDefault();
      };

      draw();
    }
  }, [])
  
  return (
    <canvas
      id="system-canvas"
      className="bg-[#1d1e22]"
      ref={canvasRef}
    />
  )
}