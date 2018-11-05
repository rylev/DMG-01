// This is a hack around a bug in the typescript webpack loaders.
// For some reason doing a dynamic import inside of typescript does not work
export default function _import() {
  return import("lib-dmg-01-js")
}