/** Minimal declarations for built-in Node APIs; no runtime or npm dependency. */
declare module 'node:crypto' {
  interface Hash { update(data:string|Uint8Array):Hash; digest(encoding:'hex'):string; }
  export function createHash(name:string):Hash;
}
declare module 'node:fs' {
  export function readFileSync(path:string|URL):Uint8Array;
  export function readFileSync(path:string|URL,encoding:'utf8'):string;
  export function writeFileSync(path:string|URL,data:string|Uint8Array):void;
  export function mkdirSync(path:string|URL,options:{recursive:true}):string|undefined;
}
declare module 'node:path' {
  export function resolve(...paths:string[]):string;
  export function dirname(path:string):string;
  export function join(...paths:string[]):string;
}
declare module 'node:url' { export function fileURLToPath(url:string|URL):string; }
declare const process:{argv:string[];cwd():string;version:string;exitCode:number};

declare module 'node:util' {
  export const types:{isProxy(value:unknown):boolean;isUint8Array(value:unknown):value is Uint8Array};
}
