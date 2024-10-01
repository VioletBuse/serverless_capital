use runtime::Runtime;

mod backend;
mod runtime;
mod tenant;

fn main() {
    let script = r"
    export default {
        event: (event, trading, storage) => {
            return trading.tradingApiName;
        }
    }
    ";

    println!("script: {}", script);

    Runtime::initialize();
    let result = Runtime::run(script);

    println!("result: {}", result);
}
