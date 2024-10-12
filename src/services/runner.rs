#[tarpc::service]
pub trait Runner {
    async fn handle_event() -> Option<()>;
}
