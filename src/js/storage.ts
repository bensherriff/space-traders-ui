import { Store } from "tauri-plugin-store-api";

const store = new Store(`store.json`);

/**
 * Retrieve the cached token.
 * @returns cached token
 */
export function getToken() {
  return getLocalStorage("token");
}

/**
 * Store the token to be cached.
 * @param {string} token 
 */
export function setToken(token: string) {
  setLocalStorage("token", token);
}

export async function getSystems() {
  const systems: string | null = await store.get("systems");
  if (!systems) {
    return {};
  }
  return JSON.parse(systems) || {};
}

export async function hasSystems(): Promise<boolean> {
  return await store.has("systems") || false;
}

export async function setSystems(systems: any) {
  await store.set("systems", JSON.stringify(systems));
  await store.save();
}

export function getLocalStorage(key: string): string {
  return localStorage.getItem(key) || "";
}

export function setLocalStorage(key: string, value: string) {
  localStorage.setItem(key, value);
}

export function getSessionStorage(key: string): string {
  return sessionStorage.getItem(key) || "";
}

export function setSessionStorage(key: string, value: string) {
  sessionStorage.setItem(key, value);
}