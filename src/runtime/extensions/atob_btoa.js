const { core } = Deno;

globalThis.atob = (str) => core.ops.atob(str);
globalThis.btoa = (str) => core.ops.btoa(str);
