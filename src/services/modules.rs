use crate::tenant::Tenant;

#[tarpc::service]
pub trait ModuleServer {
    async fn get_source(tenant: Tenant, module_specifier: String) -> Option<Vec<u8>>;
}
