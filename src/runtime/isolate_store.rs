use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use tokio::sync::Mutex;

use crate::{backend::Backend, tenant::Tenant};

use super::tenant_isolate::TenantIsolate;

type Store = Arc<Vec<Arc<Mutex<HashMap<Tenant, TenantIsolate>>>>>;

#[derive(Clone)]
pub struct IsolateStore {
    store: Store,
    backend: Backend,
}

impl IsolateStore {
    pub fn new(shards: usize, backend: Backend) -> Self {
        let mut db = Vec::with_capacity(shards);

        for _ in 0..shards {
            db.push(Arc::new(Mutex::new(HashMap::new())));
        }

        Self {
            store: Arc::new(db),
            backend,
        }
    }
    fn get_shard(&self, tenant: &Tenant) -> Arc<Mutex<HashMap<Tenant, TenantIsolate>>> {
        self.store[hash(tenant) % self.store.len()].clone()
    }
    async fn delete_isolate(&self, tenant: &Tenant) {
        let shard = self.get_shard(tenant);
        let _ = shard.lock().await.remove(tenant);
    }
    async fn ensure_isolate(&self, tenant: &Tenant) {
        let shard = self.get_shard(tenant);
        let mut map = shard.lock().await;
        let entry = map.get(tenant);

        if entry.is_none() {
            let new_isolate = TenantIsolate::new(tenant.clone(), self.backend.clone())
                .await
                .unwrap();
            map.insert(tenant.clone(), new_isolate);
        }
    }
    pub async fn handle_event(&self, tenant: &Tenant) -> anyhow::Result<String> {
        self.ensure_isolate(tenant).await;
        let shard = self.get_shard(tenant);
        let mut map = shard.lock().await;
        let isolate = map.get_mut(tenant).unwrap();

        let result = isolate.handle_event().await?;

        return Ok(result);
    }
    pub async fn handle_signal(&self, tenant: &Tenant) -> anyhow::Result<String> {
        self.ensure_isolate(tenant).await;
        let shard = self.get_shard(tenant);
        let mut map = shard.lock().await;
        let isolate = map.get_mut(tenant).unwrap();

        let result = isolate.handle_signal().await?;

        return Ok(result);
    }
    pub async fn handle_scheduled(&self, tenant: &Tenant) -> anyhow::Result<String> {
        self.ensure_isolate(tenant).await;
        let shard = self.get_shard(tenant);
        let mut map = shard.lock().await;
        let isolate = map.get_mut(tenant).unwrap();

        let result = isolate.handle_scheduled().await?;

        return Ok(result);
    }
}

fn hash<T: Hash>(to_hash: &T) -> usize {
    let mut s = DefaultHasher::new();
    to_hash.hash(&mut s);
    s.finish() as usize
}
