/**
 * Retrieve the cached agent.
 * @returns cached agent
 */
export function getAgent() {
  const agent = getSessionStorage("agent");
  if (!agent) {
    return {}
  }
  return JSON.parse(agent);
}

/**
 * Store the agent to be cached.
 * @param {*} agent 
 */
export function setAgent(agent) {
  setSessionStorage("agent", JSON.stringify(agent));
}

/**
 * Retrieve the cached token.
 * @returns cached token
 */
export function getToken() {
  const token = getLocalStorage("token");
  if (!token) {
    return ""
  }
  return token;
}

/**
 * Store the token to be cached.
 * @param {string} token 
 */
export function setToken(token) {
  setLocalStorage("token", token);
}

export function hasFaction(key) {
  const faction = getLocalStorage(`cached_faction_${key}`);
  if (faction) {
    return true
  }
  return false
}

/**
 * Retrieve the cached faction.
 * @param {string} key 
 * @returns cached faction
 */
export function getFaction(key) {
  const faction = getLocalStorage(`cached_faction_${key}`);
  if (!faction) {
    return {}
  }
  return JSON.parse(faction);
}

/**
 * Store the faction to be cached.
 * @param {*} key 
 * @param {*} faction 
 */
export function setFaction(key, faction) {
  if (key && faction) {
    setLocalStorage(`cached_faction_${key}`, JSON.stringify(faction));
  }
}

export function hasShip(key) {
  const ship = getLocalStorage(`cached_ship_${key}`);
  if (ship) {
    return true
  }
  return false
}

/**
 * Retrieve the cached ship.
 * @param {string} key 
 * @returns cached ship
 */
export function getShip(key) {
  const ship = getLocalStorage(`cached_ship_${key}`);
  if (!ship) {
    return {}
  }
  return JSON.parse(ship);
}

/**
 * Store the ship to be cached.
 * @param {*} key 
 * @param {*} ship 
 */
export function setShip(key, ship) {
  if (key && ship) {
    setLocalStorage(`cached_ship_${key}`, JSON.stringify(ship));
  }
}

/**
 * Retrieve the cached shipyard.
 * @param {*} key 
 * @returns 
 */
export function getShipyard(key) {
  const shipyard = getLocalStorage(`cached_shipyard_${key}`);
  if (!shipyard) {
    return {}
  }
  return JSON.parse(shipyard);
}

/**
 * Store the shipyard to be cached.
 * @param {*} key 
 * @param {*} shipyard 
 */
export function setShipyard(key, shipyard) {
  if (key && shipyard) {
    setLocalStorage(`cached_shipyard_${key}`, JSON.stringify(shipyard));
  }
}

/**
 * Retrieve the cached market.
 * @param {*} key 
 * @returns 
 */
export function getMarket(key) {
  const market = getLocalStorage(`cached_market_${key}`);
  if (!market) {
    return {}
  }
  return JSON.parse(market);
}

/**
 * Store the market to be cached.
 * @param {*} key 
 * @param {*} market 
 */
export function setMarket(key, market) {
  if (key && market) {
    setLocalStorage(`cached_market_${key}`, JSON.stringify(market));
  }
}

export function hasSystem(key) {
  const system = getLocalStorage(`cached_system_${key}`);
  if (system) {
    return true;
  }
  return false;
}

/**
 * Retrieve the cached system.
 * @param {*} key 
 * @returns 
 */
export function getSystem(key) {
  const system = getLocalStorage(`cached_system_${key}`);
  if (!system) {
    return {}
  }
  return JSON.parse(system);
}

/**
 * Store the system to be cached.
 * @param {*} key 
 * @param {*} system 
 */
export function setSystem(key, system) {
  if (key && system) {
    setLocalStorage(`cached_system_${key}`, JSON.stringify(system));
  }
}

export function hasWaypoint(key) {
  const waypoint = getLocalStorage(`cached_waypoint_${key}`);
  if (waypoint) {
    return true;
  }
  return false;
}

/**
 * Retrieve the cached waypoint.
 * @param {*} key 
 * @returns 
 */
export function getWaypoint(key) {
  const waypoint = getLocalStorage(`cached_waypoint_${key}`);
  if (!waypoint) {
    return {}
  }
  return JSON.parse(waypoint);
}

/**
 * Store the waypoint to be cached.
 * @param {*} key 
 * @param {*} waypoint 
 */
export function setWaypoint(key, waypoint) {
  if (key && waypoint) {
    setLocalStorage(`cached_waypoint_${key}`, JSON.stringify(waypoint));
  }
}

export function getLocalStorage(key) {
  return localStorage.getItem(key);
}

export function setLocalStorage(key, value) {
  localStorage.setItem(key, value);
}

export function getSessionStorage(key) {
  return sessionStorage.getItem(key);
}

export function setSessionStorage(key, value) {
  sessionStorage.setItem(key, value);
}