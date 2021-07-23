# Wasm pack experiment showing shared memory

The example WASM library show cases Crossbeam Channels and shared HashMap usage between workers.

1. Install [wasm-pack](https://github.com/rustwasm/wasm-pack)
2. Install [deno](https://deno.land/) for static File HTTP server, see [file-server-deno.ts](./file-server-deno.ts) <sup>1</sup>
3. Run `wasm-pack build`
4. Run `deno run --allow-run --allow-net --allow-read file-server-deno.ts`
5. Navigate to `http://localhost:8000`
6. Open a DevTools to see the communication in console

## How it works?

It initalizes only *one* `WebAssembly.Memory` object and shares it between the workers. See [index.js](./index.js) and [index.js](./worker.js) for details.


## Footnotes

1: If you don't want Deno, you still need a file server that is capable of setting headers `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`, otherwise SharedArrayBuffer is not defined. [See documentation.](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)