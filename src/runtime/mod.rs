mod templates;

use std::{collections::HashMap, sync::Once};

static INIT: Once = Once::new();

pub struct Runtime {
    isolates: HashMap<u64, v8::OwnedIsolate>,
}

impl Runtime {
    pub fn initialize() -> Self {
        INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        Self {
            isolates: HashMap::new(),
        }
    }
    pub fn run(source_code: &str) -> String {
        let backend = crate::backend::Backend {};
        let tenant = crate::tenant::Tenant {};
        let event = templates::event::Event::YOLO;

        let isolate = &mut v8::Isolate::new(Default::default());

        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope, Default::default());
        let scope = &mut v8::ContextScope::new(scope, context);

        let source = v8::String::new(scope, source_code).unwrap();
        let script_name = v8::String::new(scope, "scwipt_lol").unwrap().into();
        let script_origin = v8::ScriptOrigin::new(
            scope,
            script_name,
            0,
            0,
            false,
            0,
            None,
            false,
            false,
            true,
            None,
        );

        let compile_source = &mut v8::script_compiler::Source::new(source, Some(&script_origin));
        let module = v8::script_compiler::compile_module(scope, compile_source).unwrap();

        module
            .instantiate_module(scope, |_, _, _, m| Some(m))
            .unwrap();
        module.evaluate(scope).unwrap();

        let default_key = v8::String::new(scope, "default").unwrap();
        let default = module
            .get_module_namespace()
            .to_object(scope)
            .unwrap()
            .get(scope, default_key.into())
            .unwrap()
            .to_object(scope)
            .unwrap();

        let globalkey = v8::String::new(scope, "hello").unwrap();
        let value = v8::String::new(scope, "haiii >.<").unwrap();
        context
            .global(scope)
            .set(scope, globalkey.into(), value.into());

        let key = v8::String::new(scope, "event").unwrap();
        let fxn = default.get(scope, key.into()).unwrap();
        let function = v8::Local::<v8::Function>::try_from(fxn).unwrap();

        let functionname = v8::String::new(scope, "haii_funni_function_name").unwrap();

        let event_object_template =
            event.create_event_object_template(scope, backend.clone(), tenant.clone());
        let event_object = event_object_template.new_instance(scope).unwrap();

        let trading_object_template =
            templates::trading::create_trading_api_template(scope, backend.clone(), tenant.clone());
        let trading_object = trading_object_template.new_instance(scope).unwrap();

        let storage_object_template =
            templates::storage::create_storage_api_template(scope, backend.clone(), tenant.clone());
        let storage_object = storage_object_template.new_instance(scope).unwrap();

        let params: Vec<v8::Local<'_, v8::Value>> = vec![
            event_object.into(),
            trading_object.into(),
            storage_object.into(),
        ];

        let result = function
            .call(scope, functionname.into(), params.as_slice())
            .unwrap();

        result.to_rust_string_lossy(scope)
    }
}
