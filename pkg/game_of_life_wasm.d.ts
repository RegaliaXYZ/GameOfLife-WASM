/* tslint:disable */
/* eslint-disable */
/**
* @param {number} width
* @param {number} height
* @param {number} percentage
*/
export function newGame(width: number, height: number, percentage: number): void;
/**
* @returns {string}
*/
export function getState(): string;
/**
* @returns {string}
*/
export function getNextState(): string;
/**
* @param {number} x
* @param {number} y
*/
export function toggleState(x: number, y: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly newGame: (a: number, b: number, c: number) => void;
  readonly getState: (a: number) => void;
  readonly getNextState: (a: number) => void;
  readonly toggleState: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
