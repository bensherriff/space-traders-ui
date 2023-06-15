import { useEffect } from "react"

//TODO https://cloudfour.com/thinks/generating-svg-solar-systems-part-1-setting-the-scene/
export default function SystemSVG({system}) {

  const width = 1000;
  const height = 1000;

  useEffect(() => {
    draw();
  }, []);

  function draw() {
    let starSize = 50;
    let markup = drawStar(starSize);

    document.querySelector(".js-svg-wrapper").innerHTML = markup;
  }

  function drawStar(size) {
    return `
      <circle 
        cx="${width/2}" 
        cy="${height/2}" 
        r="${size}" 
        fill="#f7d916"
      />
    `
  }

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      // width="200"
      // height="200"
      viewBox="0 0 1000 1000"
      style={{background: "#000"}}
      role="img"
      aria-labelledby="solarSystemTitle"
      aria-describedby="solarSystemDescription"
    >
      <title id="solarSystemTitle">
        {system.symbol} Solar System
      </title>
      <desc id="solarSystemDescription">
        
      </desc>

      <g class="js-svg-wrapper"></g>

      <style></style>
    </svg>
  )
}