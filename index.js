import init from "./runner.js"

async function load(wasmPath) {
    const response = await fetch(wasmPath);
    let receivedLength = 0;
    let chunks = new Array();
    const reader = response.body.getReader();

    while (true) {
        const { done, value } = await reader.read();
            if (done) {
                break;
            } else {
                receivedLength += value.byteLength;
                chunks.push(value);
            }
    }
    let wasmBytes = new Uint8Array(receivedLength);
    let pos = 0;
    for (let chunk of chunks) {
        wasmBytes.set(chunk, pos);
        pos += chunk.byteLength;
    }

    await init(wasmBytes);
}

document.addEventListener('contextmenu', event => event.preventDefault());

load('/runner.wasm');
