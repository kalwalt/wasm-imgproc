import * as wasm from "wasm_imgproc";

console.log(wasm);
let img = wasm.image_create(100, 100);
console.log(img);
let canvas = document.getElementById('wasmCanvas');
let ctx = canvas.getContext('2d');
let imageData = ctx.createImageData(100,100)
console.log(imageData);
imageData.data.set(img);
ctx.putImageData(imageData, 0, 0);
