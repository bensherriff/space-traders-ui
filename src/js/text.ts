export function lowercase(text: string): string {
  return text.toLowerCase();
}

export function capitalize(text: string): string {
  let words = text.split(/[\s_]+/);
  var text = words.map(word => {
    return word[0].toUpperCase() + lowercase(word.substring(1));
  }).join(" ");
  return text;
}

export function currency(credits: number): string {
  let formatter = new Intl.NumberFormat();
  // return `${formatter.format(credits)} ${'\u2124'}`
  return `${formatter.format(credits)}`
}

export function date(text: string): string {
  return new Date(text).toLocaleDateString('en-us', {year: "numeric", month: "numeric", day: "numeric", second: "2-digit", minute: "2-digit", hour: "2-digit"});
}

interface TypeColor {
  text: string
  textTW: string
  hovetTW: string
  bg: string
  bgTW: string
  bghovetTWTW: string
}

export function systemTypeColor(type: string): TypeColor {
  if (type == 'NEUTRON_STAR') {
    return {
      text: "#f8fafc",
      bg: "#475569",
      textTW: "text-s-neutron-t",
      hovetTW: "",
      bgTW: "bg-s-neutron-b",
      bghovetTWTW: ""
    }
  } else if (type == 'RED_STAR') {
    return {
      text: "#fef2f2",
      bg: "#b91c1c",
      textTW: "text-s-red-t",
      hovetTW: "",
      bgTW: "bg-s-red-b",
      bghovetTWTW: ""
    }
  } else if (type == 'ORANGE_STAR') {
    return {
      text: "#431407",
      bg: "#fdba74",
      textTW: "text-s-orange-t",
      hovetTW: "",
      bgTW: "bg-s-orange-b",
      bghovetTWTW: ""
    }
  } else if (type == 'BLUE_STAR') {
    return {
      text: "#083344",
      bg: "#22d3ee",
      textTW: "text-s-blue-t",
      hovetTW: "",
      bgTW: "bg-s-blue-b",
      bghovetTWTW: ""
    }
  } else if (type == 'YOUNG_STAR') {
    return {
      text: "#022c22",
      bg: "#6ee7b7",
      textTW: "text-s-young-t",
      hovetTW: "",
      bgTW: "bg-s-young-b",
      bghovetTWTW: ""
    }
  } else if (type == 'WHITE_DWARF') {
    return {
      text: "#030712",
      bg: "#f9fafb",
      textTW: "text-s-white-t",
      hovetTW: "",
      bgTW: "bg-s-white-b",
      bghovetTWTW: ""
    }
  } else if (type == 'BLACK_HOLE') {
    return {
      text: "#f9fafb",
      bg: "#030712",
      textTW: "text-s-black-t",
      hovetTW: "",
      bgTW: "bg-s-black-b",
      bghovetTWTW: ""
    }
  } else if (type == 'HYPERGIANT') {
    return {
      text: "#2e1065",
      bg: "#c4b5fd",
      textTW: "text-s-hyper-t",
      hovetTW: "",
      bgTW: "bg-s-hyper-b",
      bghovetTWTW: ""
    }
  } else if (type == 'NEBULA') {
    return {
      text: "#fdf2f8",
      bg: "#db2777",
      textTW: "text-s-nebula-t",
      hovetTW: "",
      bgTW: "bg-s-nebula-b",
      bghovetTWTW: ""
    }
  } else if (type == 'UNSTABLE') {
    return {
      text: "#eef2ff",
      bg: "#4f46e5",
      textTW: "text-s-unstable-t",
      hovetTW: "",
      bgTW: "bg-s-unstable-b",
      bghovetTWTW: ""
    }
  } else {
    return {
      text: "#FFFFFF",
      bg: "#FFFFFF",
      textTW: "text-text-white",
      hovetTW: "#FFFFFF",
      bgTW: "bg-bg-black",
      bghovetTWTW: "bg-black"
    }
  }
}

export function waypointTypeColor(type: string): TypeColor {  
  if (type == 'PLANET') {
    return {
      text: "#ecfdf5",
      bg: "#059669",
      textTW: "text-w-planet-t",
      hovetTW: "",
      bgTW: "bg-w-planet-b",
      bghovetTWTW: ""
    }
  } else if (type == 'GAS_GIANT') {
    return {
      text: "#fff7ed",
      bg: "#ea580c",
      textTW: "text-w-gas-t",
      hovetTW: "",
      bgTW: "bg-w-gas-b",
      bghovetTWTW: ""
    }
  } else if (type == 'MOON') {
    return {
      text: "#f8fafc",
      bg: "#64748b",
      textTW: "text-w-moon-t",
      hovetTW: "",
      bgTW: "bg-w-moon-b",
      bghovetTWTW: ""
    }
  } else if (type == 'ORBITAL_STATION') {
    return {
      text: "#fdf4ff",
      bg: "#c026d3",
      textTW: "text-w-orbital-t",
      hovetTW: "",
      bgTW: "bg-w-orbital-b",
      bghovetTWTW: ""
    }
  } else if (type == 'JUMP_GATE') {
    return {
      text: "#030712",
      bg: "#f9fafb",
      textTW: "text-w-jump-t",
      hovetTW: "",
      bgTW: "bg-w-jump-b",
      bghovetTWTW: ""
    }
  } else if (type == 'ASTEROID_FIELD') {
    return {
      text: "#1a2e05",
      bg: "#bef264",
      textTW: "text-w-asteroid-t",
      hovetTW: "",
      bgTW: "bg-w-asteroid-b",
      bghovetTWTW: ""
    }
  } else if (type == 'NEBULA') {
    return {
      text: "#422006",
      bg: "#fde047",
      textTW: "text-w-nebula-t",
      hovetTW: "",
      bgTW: "bg-w-nebula-b",
      bghovetTWTW: ""
    }
  } else if (type == 'DEBRIS_FIELD') {
    return {
      text: "#083344",
      bg: "#67e8f9",
      textTW: "text-w-debris-t",
      hovetTW: "",
      bgTW: "bg-w-debris-b",
      bghovetTWTW: ""
    }
  } else if (type == 'GRAVITY_WELL') {
    return {
      text: "#f9fafb",
      bg: "#030712",
      textTW: "text-w-gravity-t",
      hovetTW: "",
      bgTW: "bg-w-gravity-b",
      bghovetTWTW: ""
    }
  } else {
    return {
      text: "#FFFFFF",
      bg: "#FFFFFF",
      textTW: "text-text-white",
      hovetTW: "text-white",
      bgTW: "bg-bg-black",
      bghovetTWTW: "bg-black"
    }
  }
}