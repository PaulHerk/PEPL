/* tslint:disable */
/* eslint-disable */
/**
* @param {string} content
* @returns {string}
*/
export function main(content: string): string;
/**
*/
export enum ErrorkindOnInterpreter {
  TackNotFound,
  NoItemInTack,
  InvalidNumber,
  NoSuchCharacter,
}
/**
*/
export enum ErrorkindOnLexer {
  InvalidToken,
  NoContent,
  NoValuePut,
  BackslashAfterLastCommand,
  NoOpeningLoop,
  NoEndOrElseIf,
  NoClosingLoop,
  NoOpeningIf,
}
/**
*/
export class Error {
  free(): void;
/**
*/
  error_kind: boolean;
/**
*/
  error_on_interpreter?: ErrorOnInterpreter;
/**
*/
  error_on_lexer?: ErrorOnLexer;
}
/**
*/
export class ErrorOnInterpreter {
  free(): void;
/**
*/
  kind: number;
/**
*/
  position: number;
}
/**
*/
export class ErrorOnLexer {
  free(): void;
/**
*/
  kind: number;
/**
*/
  position: number;
}
/**
*/
export class Options {
  free(): void;
/**
*/
  debug: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_get_options_debug: (a: number) => number;
  readonly __wbg_set_options_debug: (a: number, b: number) => void;
  readonly __wbg_error_free: (a: number) => void;
  readonly __wbg_get_error_error_on_lexer: (a: number) => number;
  readonly __wbg_set_error_error_on_lexer: (a: number, b: number) => void;
  readonly __wbg_get_error_error_on_interpreter: (a: number) => number;
  readonly __wbg_set_error_error_on_interpreter: (a: number, b: number) => void;
  readonly __wbg_get_error_error_kind: (a: number) => number;
  readonly __wbg_set_error_error_kind: (a: number, b: number) => void;
  readonly main: (a: number, b: number, c: number) => void;
  readonly __wbg_get_erroroninterpreter_kind: (a: number) => number;
  readonly __wbg_set_erroroninterpreter_kind: (a: number, b: number) => void;
  readonly __wbg_get_erroroninterpreter_position: (a: number) => number;
  readonly __wbg_set_erroroninterpreter_position: (a: number, b: number) => void;
  readonly __wbg_get_erroronlexer_kind: (a: number) => number;
  readonly __wbg_set_erroronlexer_kind: (a: number, b: number) => void;
  readonly __wbg_options_free: (a: number) => void;
  readonly __wbg_erroroninterpreter_free: (a: number) => void;
  readonly __wbg_erroronlexer_free: (a: number) => void;
  readonly __wbg_get_erroronlexer_position: (a: number) => number;
  readonly __wbg_set_erroronlexer_position: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
