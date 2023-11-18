module.exports = {};

function f() {
  if (!process.nxpkgpack) {
    throw new Error("Nxpkgpack is not enabled");
  }
  if (process.env.NODE_ENV !== "development") {
    throw new Error("NODE_ENV is not development");
  }
}

f();

// if (f.toString().includes("process.nxpkgpack")) {
//   throw new Error("process.nxpkgpack is not replaced");
// }
