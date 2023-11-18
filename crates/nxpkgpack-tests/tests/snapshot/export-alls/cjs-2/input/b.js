// b.js
export * from "./c";
// This would not be handled, but still need __nxpkgpack__cjs__
// as there are properties dynamically added by __nxpkgpack__cjs__ in c.js
