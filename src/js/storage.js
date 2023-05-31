export function getSessionAgent() {
  const agent = sessionStorage.getItem("agent");
  if (!agent) {
    return {}
  }
  return JSON.parse(agent);
}

export function setSessionAgent(agent) {
  sessionStorage.setItem("agent", JSON.stringify(agent));
}

export function getSessionToken() {
  const token = sessionStorage.getItem("token");
  if (!token) {
    return ""
  }
  return token;
}

export function setSessionToken(token) {
  sessionStorage.setItem("token", token);
}

export function getShip(key) {
  const ship = getLocalStorage(`cached_ship_${key}`);
  if (!ship) {
    return {}
  }
  return JSON.parse(ship);
}

export function setShip(key, ship) {
  if (key && ship) {
    setLocalStorage(`cached_ship_${key}`, JSON.stringify(ship));
  }
}

export function getLocalStorage(key) {
  return localStorage.getItem(key);
}

export function setLocalStorage(key, value) {
  localStorage.setItem(key, value);
}