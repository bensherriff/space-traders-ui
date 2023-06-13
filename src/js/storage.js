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