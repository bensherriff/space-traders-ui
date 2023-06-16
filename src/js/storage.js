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

export function getSystems() {
  const systems = getLocalStorage("systems");
  if (!systems) {
    return {};
  }
  return JSON.parse(systems);
}

export function hasSystems() {
  const systems = getLocalStorage("systems");
  if (systems) {
    return true;
  } else {
    return false;
  }
}

export function setSystems(systems) {
  setLocalStorage("systems", JSON.stringify(systems));
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