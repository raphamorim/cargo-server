fetch("example.wasm").then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes)
).then(wasm => {
  const result = wasm.instance.exports.subtracao(5, 1); // 4
  console.log(`5 - 1 is: ${result}`);
});