export function shortenString(str: string, length: number): string {
  if (str.length > length) {
    return str.slice(0, length/2) + '...' + str.slice(str.length - length/2, str.length);
  }

  return str;
}