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