import { Store } from "tauri-plugin-store-api";
import { localDataDir, sep } from '@tauri-apps/api/path';

const localDataDirPath = await localDataDir();
const store = new Store(`store.json`);

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

export async function getSystems() {
  const systems = await store.get("systems");
  if (!systems) {
    return {};
  }
  return JSON.parse(systems);
}

export async function hasSystems() {
  console.log('here', store.has("systems"));
  await store.has("systems") || false;
}

export async function setSystems(systems) {
  await store.set("systems", JSON.stringify(systems));
  await store.save();
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