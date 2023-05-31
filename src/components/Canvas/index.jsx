import { useEffect, useRef } from "react"

class SolarObject {
  constructor(context, x, y, type) {
    this.context = context;
    this.x = x,
    this.y = y,
    this.type = type
  }

  draw() {
    this.context.beginPath()
    this.context.arc(this.x, this.y, 10, 0, Math.PI * 2, false);
    this.context.fillStyle = '#FFFFFF';
    this.context.fill();
    this.context.closePath();
  }

  update() {
    this.draw();
  }
}

export function SystemMap({system}) {
  const canvasRef = useRef(null)
  console.log(system);
  
  // function draw(ctx) {
  //   ctx.fillStyle = '#000000';
  //   ctx.beginPath();
  //   ctx.arc(50, 100, 20, 0, 2*Math.PI);
  //   ctx.fill();
  // }

  useEffect(() => {
    const canvas = canvasRef.current;
    const context = canvas.getContext('2d');

    const height = 200; // context.canvas.width
    const width = 200; // context.canvas.height

    context.fillStyle = '#000000';
    context.fillRect(0, 0, width, height);

    let sun = new SolarObject(context, width/2, height/2, 'test');
    sun.draw();

  }, [])
  
  return (
    <div className="h-full w-full">
      <canvas ref={canvasRef}/>
    </div>
  )
}