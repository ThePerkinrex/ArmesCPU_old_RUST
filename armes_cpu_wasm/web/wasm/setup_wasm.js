import { default as init } from './armes_cpu_lib.js';
import * as wasm from './armes_cpu_lib.js';
import {main} from "../index.js";

async function run() {
    // First up we need to actually load the wasm file, so we use the
    // default export to inform it where the wasm file is located on the
    // server, and then we wait on the returned promise to wait for the
    // wasm to be loaded.
    //
    // Note that instead of a string here you can also pass in an instance
    // of `WebAssembly.Module` which allows you to compile your own module.
    // Also note that the promise, when resolved, yields the wasm module's
    // exports which is the same as importing the `*_bg` module in other
    // modes
    await init('/wasm/armes_cpu_lib_bg.wasm');
    // And afterwards we can use all the functionality defined in wasm.
    main(wasm);
}

run();