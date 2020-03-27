const WASM_FILE = fetch('/lisp_interpreter.wasm');

const InitLisp = async (lisp_log_function) => {
  let log_str_wasm = () => console.error("log_str_memory isn't set yet")
  
  const importObject = {
    imports: {
      log_str: (test) => log_str_wasm(test)
    },
  };

  const exports = await WebAssembly.instantiateStreaming(WASM_FILE, importObject).then((wasmImport) => {
    log_str_wasm = function(ptr) {
      let memory_array = new Uint8Array(wasmImport.instance.exports.memory.buffer);
  
      let i = 0;
      for (;memory_array[ptr + i] != 0; i += 1);

      lisp_log_function(new TextDecoder("utf-8").decode(memory_array.slice(ptr, ptr + i)));
    };

    return wasmImport.instance.exports;
  });

  return function(lispProgram) {
    const { execute_bytes_array, allocateUint8Array } = exports;
    const paramArrayJS = new TextEncoder("utf-8").encode(lispProgram);

    paramPtr = allocateUint8Array(paramArrayJS.length);
    paramWasmArray = new Uint8Array(exports.memory.buffer, paramPtr, paramArrayJS.length);
    paramWasmArray.set(paramArrayJS);
  
    execute_bytes_array(paramPtr, paramArrayJS.length);
  };
}

(async function () {
  const Lisp = await InitLisp(console.log);

  Lisp(`
    (def fibo
          (λ n
            (if (< n 2)
              n
              (+ (fibo (- n 1))
                (fibo (- n 2))))))
    (print (fibo 5))
  `)
})();
