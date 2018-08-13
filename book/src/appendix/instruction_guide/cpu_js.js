
(function() {
    var wasm;
    const __exports = {};


    const stack = [];

    const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

    function getObject(idx) {
        if ((idx & 1) === 1) {
            return stack[idx >> 1];
        } else {
            const val = slab[idx >> 1];

            return val.obj;

        }
    }

    let slab_next = slab.length;

    function dropRef(idx) {

        idx = idx >> 1;
        if (idx < 4) return;
        let obj = slab[idx];

        obj.cnt -= 1;
        if (obj.cnt > 0) return;

        // If we hit 0 then free up our space in the slab
        slab[idx] = slab_next;
        slab_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropRef(idx);
        return ret;
    }
    /**
    */
    __exports.Target = Object.freeze({ A:0,B:1,C:2,D:3,E:4,F:5,H:6,L:7,AF:8,BC:9,DE:10,HL:11, });
    /**
    */
    __exports.Register = Object.freeze({ A:0,B:1,C:2,D:3,E:4,F:5,H:6,L:7,AF:8,BC:9,DE:10,HL:11, });
    /**
    * @param {CPU} arg0
    * @param {number} arg1
    * @returns {CPU}
    */
    __exports.add = function(arg0, arg1) {
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error('Attempt to use a moved value');
        }
        arg0.ptr = 0;
        return CPU.__construct(wasm.add(ptr0, arg1));
    };

    class ConstructorToken {
        constructor(ptr) {
            this.ptr = ptr;
        }
    }
    /**
    */
    class CPU {

        static __construct(ptr) {
            return new CPU(new ConstructorToken(ptr));
        }

        constructor(...args) {
            if (args.length === 1 && args[0] instanceof ConstructorToken) {
                this.ptr = args[0].ptr;
                return;
            }

            // This invocation of new will call this constructor with a ConstructorToken
            let instance = CPU.new(...args);
            this.ptr = instance.ptr;
        }
        free() {
            const ptr = this.ptr;
            this.ptr = 0;
            wasm.__wbg_cpu_free(ptr);
        }
        /**
        * @returns {CPU}
        */
        static new() {
            return CPU.__construct(wasm.cpu_new());
        }
        /**
        * @param {number} arg0
        * @param {number} arg1
        * @returns {void}
        */
        set_register(arg0, arg1) {
            if (this.ptr === 0) {
                throw new Error('Attempt to use a moved value');
            }
            return wasm.cpu_set_register(this.ptr, arg0, arg1);
        }
        /**
        * @returns {any}
        */
        to_json() {
            if (this.ptr === 0) {
                throw new Error('Attempt to use a moved value');
            }
            return takeObject(wasm.cpu_to_json(this.ptr));
        }
    }
    __exports.CPU = CPU;

    function addHeapObject(obj) {
        if (slab_next === slab.length) slab.push(slab.length + 1);
        const idx = slab_next;
        const next = slab[idx];

        slab_next = next;

        slab[idx] = { obj, cnt: 1 };
        return idx << 1;
    }

    let cachedDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__wbindgen_json_parse = function(ptr, len) {
        return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));
    };

    __exports.__wbindgen_throw = function(ptr, len) {
        throw new Error(getStringFromWasm(ptr, len));
    };

    function init(wasm_path) {
        return fetch(wasm_path)
        .then(response => response.arrayBuffer())
        .then(buffer => WebAssembly.instantiate(buffer, { './cpu_js': __exports }))
        .then(({instance}) => {
            wasm = init.wasm = instance.exports;
            return;
        });
    };
    self.wasm_bindgen = Object.assign(init, __exports);
})();

