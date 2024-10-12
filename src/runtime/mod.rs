use isolate_store::IsolateStore;

use crate::{backend::Backend, tenant::Tenant};

mod extensions;
mod isolate_store;
mod module_loader;
mod tenant_isolate;

#[derive(Clone)]
pub struct Runtime {
    store: IsolateStore,
}

impl Runtime {
    pub fn new(shards: usize, backend: Backend) -> Self {
        Self {
            store: IsolateStore::new(shards, backend),
        }
    }
    pub async fn run_event(&self, tenant: &Tenant) -> anyhow::Result<String> {
        self.store.handle_event(tenant).await
    }
    pub async fn run_signal(&self, tenant: &Tenant) -> anyhow::Result<String> {
        self.store.handle_signal(tenant).await
    }
    pub async fn run_scheduled(&self, tenant: &Tenant) -> anyhow::Result<String> {
        self.store.handle_scheduled(tenant).await
    }
}
