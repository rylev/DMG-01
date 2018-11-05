export function toHex(byte: number, places: number = 2): string {
  const hex = byte.toString(16)
  const padding = places - hex.length
  return `${"0".repeat(padding > 0 ? padding : 0)}${hex}`
}