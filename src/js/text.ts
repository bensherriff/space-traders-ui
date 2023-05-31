export function lowercase(text: string) {
  return text.toLowerCase();
}

export function capitalize(text: string) {
  return text[0].toUpperCase() + lowercase(text.substring(1));
}