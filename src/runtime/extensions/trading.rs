use deno_core::{error::AnyError, extension, op2};

extension!(
    trading_api,
    ops = [exchange],
    esm_entry_point = "ext:trading_api/trading.js",
    esm = [dir "src/runtime/extensions", "trading.js"]
);

// include_js_files!(
//     trading_api
//     dir "src/runtime/extensions",
//     "trading.ts"
// );

#[op2(async)]
#[string]
async fn exchange() -> Result<String, AnyError> {
    Ok("Unknown Exchange".to_string())
}
