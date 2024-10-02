use anyhow::anyhow;
use base64::Engine;
use deno_core::{error::AnyError, extension, op2};

extension!(
    atob_btoa,
    ops = [atob, btoa],
    esm_entry_point = "ext:atob_btoa/atob_btoa.js",
    esm = [dir "src/runtime/extensions", "atob_btoa.js"]
);

#[op2]
#[string]
fn atob(#[string] ascii: String) -> Result<String, AnyError> {
    Ok(base64::engine::general_purpose::STANDARD.encode(ascii.as_bytes()))
}

#[op2]
#[string]
fn btoa(#[string] bytes: String) -> Result<String, AnyError> {
    match base64::engine::general_purpose::STANDARD.decode(bytes) {
        Ok(bytes) => String::from_utf8(bytes).map_err(|_| anyhow!("Invalid utf-8 codepoints")),
        Err(_) => Err(anyhow!("Invalid Base64 Code")),
    }
}
