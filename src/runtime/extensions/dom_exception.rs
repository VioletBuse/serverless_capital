use deno_core::extension;

extension!(
    dom_exception,
    esm_entry_point = "ext:dom_exception/dom_exception.js",
    esm = [dir "src/runtime/extensions", "dom_exception.js"]
);
