const wasm = require("./pkg/wasm_pack.js")

console.log(`The factorial of 10 is: ${wasm.factorial(BigInt(10))}`);
