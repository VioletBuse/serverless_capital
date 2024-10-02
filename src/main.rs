use runtime::Runtime;

mod backend;
mod runtime;
mod tenant;

fn main() {
    let script = r"
    export default {
        event: async (event, trading, storage) => {
            const prommy = new Promise((resolve, reject) => {
                resolve(trading.tradingApiName)
            });

            return await prommy
        }
    }
    ";

    println!("script: {}", script);

    Runtime::initialize();
    let result = Runtime::run(script);

    println!("result: {}", result);
}
