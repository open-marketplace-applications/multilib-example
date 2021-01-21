import("../pkg/index.js").then(async wasm => {
    console.log(wasm)
    alert(wasm.greet());
});
