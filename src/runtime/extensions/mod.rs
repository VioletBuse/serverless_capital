use deno_core::Extension;

mod atob_btoa;
mod dom_exception;
mod trading;

pub fn extensions_list() -> Vec<Extension> {
    vec![
        dom_exception::dom_exception::init_ops_and_esm(),
        atob_btoa::atob_btoa::init_ops_and_esm(),
        trading::trading_api::init_ops_and_esm(),
    ]
}
