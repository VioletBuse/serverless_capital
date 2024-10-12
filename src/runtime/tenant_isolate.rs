use std::rc::Rc;

use anyhow::anyhow;
use deno_core::{serde_v8, v8, JsRuntime};
use serde::Serialize;

use crate::{backend::Backend, tenant::Tenant};

use super::{extensions, module_loader};

pub struct TenantIsolate {
    runtime: JsRuntime,
    event_handler: v8::Global<v8::Function>,
    signal_handler: v8::Global<v8::Function>,
    scheduled_handler: Option<v8::Global<v8::Function>>,
}

impl TenantIsolate {
    pub async fn new(tenant: Tenant, _backend: Backend) -> anyhow::Result<Self> {
        let main_module =
            deno_core::resolve_path(tenant.module.clone(), &std::env::current_dir()?)?;

        println!("main module {main_module}");

        let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(module_loader::CustomModuleLoader::new(
                tenant.clone(),
            ))),
            extensions: extensions::extensions_list(),
            ..Default::default()
        });

        Self::set_global_variable(&mut js_runtime.handle_scope(), "tenant_id", tenant.id)?;

        let mod_id = js_runtime.load_main_es_module(&main_module).await?;
        let result = js_runtime.mod_evaluate(mod_id);

        js_runtime.run_event_loop(Default::default()).await?;

        result.await?;

        let default_export = Self::get_default_export(&mut js_runtime, mod_id)?;
        let event_handler = Self::get_function(&mut js_runtime, &default_export, "event")?;
        let signal_handler = Self::get_function(&mut js_runtime, &default_export, "signal")?;
        let scheduled_handler =
            Self::get_function(&mut js_runtime, &default_export, "scheduled").ok();

        Ok(Self {
            runtime: js_runtime,
            event_handler,
            signal_handler,
            scheduled_handler,
        })
    }
    fn set_global_variable<V>(
        scope: &mut v8::HandleScope,
        key: &str,
        value: V,
    ) -> anyhow::Result<()>
    where
        V: Serialize,
    {
        let context = scope.get_current_context();
        let global = context.global(scope);

        let key = deno_core::serde_v8::to_v8(scope, key)?;
        let value = deno_core::serde_v8::to_v8(scope, value)?;

        global.set(scope, key, value);

        Ok(())
    }

    fn get_default_export(
        runtime: &mut JsRuntime,
        // scope: &mut v8::HandleScope<'a>,
        // namespace: &v8::Global<v8::Object>,
        mod_id: usize,
    ) -> anyhow::Result<v8::Global<v8::Object>> {
        let namespace = runtime.get_module_namespace(mod_id)?;
        let scope = &mut runtime.handle_scope();
        let exports = v8::Local::new(scope, namespace);

        let key = serde_v8::to_v8(scope, "default")?;
        let default = exports
            .get(scope, key)
            .ok_or(anyhow!("No default export present"))?;
        let default = v8::Local::<v8::Object>::try_from(default)
            .map_err(|_| anyhow!("Default export is not an object."))?
            .clone();
        let default = v8::Global::new(scope, default);

        Ok(default)
    }
    fn get_function(
        // scope: &mut v8::HandleScope,
        runtime: &mut JsRuntime,
        object: &v8::Global<v8::Object>,
        key: &str,
    ) -> anyhow::Result<v8::Global<v8::Function>> {
        let scope = &mut runtime.handle_scope();
        let js_key = serde_v8::to_v8(scope, key)?;
        let object = v8::Local::new(scope, object);
        let fxn = object
            .get(scope, js_key)
            .ok_or(anyhow!("object has no property {key}"))?;
        let fxn = v8::Local::<v8::Function>::try_from(fxn)
            .map_err(|_| anyhow!("{key} is not a function"))?;
        let fxn = v8::Global::new(scope, fxn).clone();

        Ok(fxn)
    }
    pub async fn handle_event(&mut self) -> anyhow::Result<String> {
        let function_call = self.runtime.call(&self.event_handler);
        let result = self
            .runtime
            .with_event_loop_promise(function_call, Default::default())
            .await?;

        let scope = &mut self.runtime.handle_scope();
        let result = v8::Local::new(scope, result);

        return Ok(result.to_rust_string_lossy(scope));
    }
    pub async fn handle_signal(&mut self) -> anyhow::Result<String> {
        let function_call = self.runtime.call(&self.signal_handler);
        let result = self
            .runtime
            .with_event_loop_promise(function_call, Default::default())
            .await?;

        let scope = &mut self.runtime.handle_scope();
        let result = v8::Local::new(scope, result);

        return Ok(result.to_rust_string_lossy(scope));
    }
    pub async fn handle_scheduled(&mut self) -> anyhow::Result<String> {
        let function = self
            .scheduled_handler
            .as_ref()
            .ok_or(anyhow!("No scheduled handler defined"))?;

        let function_call = self.runtime.call(function);
        let result = self
            .runtime
            .with_event_loop_promise(function_call, Default::default())
            .await?;

        let scope = &mut self.runtime.handle_scope();
        let result = v8::Local::new(scope, result);

        return Ok(result.to_rust_string_lossy(scope));
    }
}
