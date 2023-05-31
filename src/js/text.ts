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

export function date(text: string): string {
  return new Date(text).toLocaleDateString('en-us', {year: "numeric", month: "numeric", day: "numeric", second: "2-digit", minute: "2-digit", hour: "2-digit"});
}