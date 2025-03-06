/* tslint:disable */
/* eslint-disable */
export class Terminal {
  free(): void;
  /**
   * Constructor for Terminal
   */
  constructor();
  /**
   * takes in the currently typed command and returns the most likely command via a kind of
   * fuzzy search that the user is trying to type.
   */
  tab_complete(command: string): string;
  /**
   * returns the next or previous command in the command history based on the dircection given.
   * only accepts "up" or "down" as direction"
   */
  cycle_command_history(direction: string): string;
  /**
   * takes in a string command and returns the output as a string
   */
  handle_command(command: string): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_terminal_free: (a: number, b: number) => void;
  readonly terminal_new: () => number;
  readonly terminal_tab_complete: (a: number, b: number, c: number) => [number, number];
  readonly terminal_cycle_command_history: (a: number, b: number, c: number) => [number, number];
  readonly terminal_handle_command: (a: number, b: number, c: number) => [number, number, number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
