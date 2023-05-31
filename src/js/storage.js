export function getAgent() {
  const agent = getSessionStorage("agent");
  if (!agent) {
    return {}
  }
  return JSON.parse(agent);
}

export function setAgent(agent) {
  setSessionStorage("agent", JSON.stringify(agent));
}

export function getToken() {
  const token = getLocalStorage("token");
  if (!token) {
    return ""
  }
  return token;
}

export function setToken(token) {
  setLocalStorage("token", token);
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

export function getSessionStorage(key) {
  return sessionStorage.getItem(key);
}

export function setSessionStorage(key, value) {
  sessionStorage.setItem(key, value);
}