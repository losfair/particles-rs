import Common from "./common";

let wasmModule: WebAssembly.Module = null;

export class RuntimeEnvironment {
    public instance: WebAssembly.Instance;
    public mem: WebAssembly.Memory;

    public constructor() {
        this.instance = null;
        this.mem = null;
    }

    public async initialize(url: string = null) {
        if(!wasmModule) {
            let code;
            if(Common.code) {
                code = Common.code.buffer as ArrayBuffer;
            } else {
                let codeResponse = await fetch(url || "particles.wasm");
                code = await codeResponse.arrayBuffer();
            }
            wasmModule = await WebAssembly.compile(code);
        }

        let instance = await WebAssembly.instantiate(
            wasmModule,
            {
                env: this.buildEnv()
            }
        );

        this.instance = instance;
        this.mem = instance.exports.memory as WebAssembly.Memory;
    }

    public buildEnv() {
        return {
            i_rand01() {
                return Math.random();
            }
        };
    }

    public allocate(len: number) {
        let m = this.instance.exports.g_alloc(len);
        return m;
    }

    public deallocate(ptr: number) {
        this.instance.exports.g_free(ptr);
    }

    public destroyCstring(ptr: number) {
        this.instance.exports.g_destroy_cstring(ptr);
    }

    public writePtr(ptr: number, data: Uint8Array, len: number) {
        let arrayView = new Uint8Array(this.mem.buffer);
        for(let i = 0; i < len; i++) {
            arrayView[ptr + i] = data[i];
        }
    }

    public readString(ptr: number, len: number = 0) : string {
        let arrayView = new Uint8Array(this.mem.buffer);
        if(!len) {
            len = 0;
            let p = ptr;
            while(arrayView[p]) {
                p++;
                len++;
            }
        }
        let result = new Uint8Array(len);
        for(let i = 0; i < len; i++) {
            result[i] = arrayView[ptr + i];
        }
        let decoder = new (window as any).TextDecoder();
        return decoder.decode(result);
    }
}

export function isPlatformSupported() {
    try {
        let _ = WebAssembly;
    } catch(e) {
        return false;
    }
    return true;
}
