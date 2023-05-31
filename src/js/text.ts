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
  return `${formatter.format(credits)} ${'\u2124'}`
}