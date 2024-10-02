mod builtins;
mod extensions;
mod module_loader;

use std::{collections::HashMap, rc::Rc, sync::Arc};

use anyhow::anyhow;
use deno_core::{
    error::AnyError,
    v8::{self, Handle},
    JsRuntime,
};
use extensions::extensions_list;
use tokio::sync::Mutex;

use crate::{backend::Backend, tenant::Tenant};

#[derive()]
pub struct Handlers {
    pub runtime: JsRuntime,
    pub event: v8::Global<v8::Function>,
    pub fetch: v8::Global<v8::Function>,
}

#[derive(Clone)]
pub struct Runtime {
    tenants: Arc<Mutex<HashMap<Tenant, Handlers>>>,
    backend: Backend,
}

impl Runtime {
    pub fn new(backend: Backend) -> Self {
        Self {
            tenants: Arc::new(Mutex::new(HashMap::new())),
            backend,
        }
    }
    pub async fn initialize_isolate(&self, tenant: Tenant) -> Result<(), AnyError> {
        let main_module =
            deno_core::resolve_path(tenant.module.clone(), &std::env::current_dir()?)?;
        let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(module_loader::SCModuleLoader::new())),
            extensions: extensions_list(),
            ..Default::default()
        });

        let (event_handler, fetch_handler) = {
            let mod_id = js_runtime.load_main_es_module(&main_module).await?;
            let result = js_runtime.mod_evaluate(mod_id);

            js_runtime.run_event_loop(Default::default()).await?;

            result.await?;

            let mod_namespace = js_runtime.get_module_namespace(mod_id)?;
            let scope = &mut js_runtime.handle_scope();
            let exports = v8::Local::new(scope, mod_namespace);

            let key = v8::String::new(scope, "default").unwrap();
            let default = exports
                .get(scope, key.into())
                .ok_or(anyhow!("No default export"))?;
            let default = v8::Local::<v8::Object>::try_from(default)?;

            let evt_handle_key = v8::String::new(scope, "event").unwrap();
            let evt_handler = default
                .get(scope, evt_handle_key.into())
                .ok_or(anyhow!("No 'event' handler defined"))?;
            let evt_handler = v8::Local::<v8::Function>::try_from(evt_handler)?;
            let evt_handler = v8::Global::new(scope, evt_handler);

            let fetch_handle_key = v8::String::new(scope, "fetch").unwrap();
            let fetch_handler = default
                .get(scope, fetch_handle_key.into())
                .ok_or(anyhow!("No 'fetch' handler defined"))?;
            let fetch_handler = v8::Local::<v8::Function>::try_from(fetch_handler)?;
            let fetch_handler = v8::Global::new(scope, fetch_handler);

            (evt_handler, fetch_handler)
        };

        let handlers = Handlers {
            runtime: js_runtime,
            event: event_handler,
            fetch: fetch_handler,
        };

        self.tenants.try_lock().unwrap().insert(tenant, handlers);

        Ok(())
    }
}
