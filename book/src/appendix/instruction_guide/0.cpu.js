(window["webpackJsonpCPU"] = window["webpackJsonpCPU"] || []).push([[0],{

/***/ "../dmg-01-js/pkg/dmg_01_js.js":
/*!*************************************!*\
  !*** ../dmg-01-js/pkg/dmg_01_js.js ***!
  \*************************************/
/*! exports provided: Target, Register, add, CPU, __wbindgen_json_parse, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Target", function() { return Target; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Register", function() { return Register; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "add", function() { return add; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "CPU", function() { return CPU; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_json_parse", function() { return __wbindgen_json_parse; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return __wbindgen_throw; });
/* harmony import */ var _dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./dmg_01_js_bg */ "../dmg-01-js/pkg/dmg_01_js_bg.wasm");
/* tslint:disable */


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
const Target = Object.freeze({ A:0,B:1,C:2,D:3,E:4,F:5,H:6,L:7,AF:8,BC:9,DE:10,HL:11, });
/**
*/
const Register = Object.freeze({ A:0,B:1,C:2,D:3,E:4,F:5,H:6,L:7,AF:8,BC:9,DE:10,HL:11, });
/**
* @param {CPU} arg0
* @param {number} arg1
* @returns {CPU}
*/
function add(arg0, arg1) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error('Attempt to use a moved value');
    }
    arg0.ptr = 0;
    return CPU.__construct(_dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["add"](ptr0, arg1));
}

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
        _dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["__wbg_cpu_free"](ptr);
    }
    /**
    * @returns {CPU}
    */
    static new() {
        return CPU.__construct(_dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["cpu_new"]());
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
        return _dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["cpu_set_register"](this.ptr, arg0, arg1);
    }
    /**
    * @returns {any}
    */
    to_json() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return takeObject(_dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["cpu_to_json"](this.ptr));
    }
}

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
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetUint8Memory = new Uint8Array(_dmg_01_js_bg__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __wbindgen_json_parse(ptr, len) {
    return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));
}

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}



/***/ }),

/***/ "../dmg-01-js/pkg/dmg_01_js_bg.wasm":
/*!******************************************!*\
  !*** ../dmg-01-js/pkg/dmg_01_js_bg.wasm ***!
  \******************************************/
/*! exports provided: memory, __heap_base, __data_end, __wbg_cpu_free, cpu_new, cpu_set_register, cpu_to_json, add */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.i];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name != "__webpack_init__") exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./dmg_01_js */ "../dmg-01-js/pkg/dmg_01_js.js");


// exec wasm module
wasmExports["__webpack_init__"]()

/***/ })

}]);
//# sourceMappingURL=0.cpu.js.map