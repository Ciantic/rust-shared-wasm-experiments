import init, { add_to_map } from "./pkg/shared_wasm_experiments.js";
const mod = await init();

// "__wbindgen_export_0" will be renamed to "memory" in future wasm-bindgen, see
// https://github.com/rustwasm/wasm-bindgen/issues/2102
const mem = mod.__wbindgen_export_0;
if (!(mem instanceof WebAssembly.Memory)) {
    console.log("mod", mod);
    throw new Error("Maybe exports have changed? Try renaming __wbindgen_export_0 to memory");
}

/**
 * Create workers
 *
 * 1. Creates a worker
 * 2. Sends an message and the shared memory to the worker
 * 3. Waits until the initialization is done
 * 4. Returns promises to the initialized workers
 *
 * @param {WebAssembly.Memory} mem
 * @param {number} count
 * @returns {Promise<Worker[]>}
 */
async function createWorkers(mem, count) {
    const workers = [...Array(count).keys()].map(
        (id) =>
            new Promise((res, _rej) => {
                const rejectedTimeout = setTimeout(() => rej("timeout"), 3000);

                // Create worker
                const worker = new Worker("./worker.js", {
                    type: "module",
                });

                // Send the shared memory to worker
                worker.postMessage({ task: "init", mem, id });

                // Wait for the worker to report back
                worker.onmessage = function (e) {
                    worker.onmessage = undefined;
                    if (e.data === "init done") {
                        clearTimeout(rejectedTimeout);
                        res(worker);
                    }
                };
            })
    );

    return await Promise.all(workers);
}

const workers = await createWorkers(mem, 4);

// Add to map from this module
add_to_map(1, "Added from main thread!");
console.log("Main thread : add_to_map", 1, "Added from main thread!");

// Add to map in the first worker
setTimeout(() => {
    workers[0].postMessage({
        task: "add_to_map",
        key: 2,
        value: "This is stored in shared memory by worker 0!",
    });
}, 10);

// Get from map in the worker 2 and 3
setTimeout(() => {
    workers[1].postMessage({ task: "get_from_map", key: 2 });
    workers[2].postMessage({ task: "get_from_map", key: 2 });
    workers[3].postMessage({ task: "get_from_map", key: 1 });
}, 1500);

// Above should print out this in the console:
// Worker 0 : add_to_map 2 Foo
// Worker 2 : get_from_map Foo
// Worker 1 : get_from_map Foo
