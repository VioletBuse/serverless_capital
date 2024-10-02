use deno_core::Extension;

mod atob_btoa;
mod trading;

pub fn extensions_list() -> Vec<Extension> {
    vec![
        atob_btoa::atob_btoa::init_ops_and_esm(),
        trading::trading_api::init_ops_and_esm(),
    ]
}
