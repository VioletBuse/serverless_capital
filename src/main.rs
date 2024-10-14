use runtime::Runtime;

mod backend;
mod runtime;
mod tenant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let script = r"
    // export default {
    //     event: async (event, trading, storage) => {
    //         const prommy = new Promise((resolve, reject) => {
    //             resolve(trading.tradingApiName)
    //         });

    //         return await prommy
    //     }
    // }
    // ";

    // println!("script: {}", script);

    let backend = backend::Backend {};
    let runtime = Runtime::new(99, backend);

    let tenant = tenant::Tenant {
        module: "./hello.ts".into(),
        id: "tenant_id_jsd2387rhdishflsjdf".into(),
    };

    let result = runtime.run_event(&tenant).await;

    println!("result: {:#?}", result);

    Ok(())
}
