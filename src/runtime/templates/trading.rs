use crate::{backend::Backend, tenant::Tenant};

pub fn create_trading_api_template<'a>(
    scope: &mut v8::HandleScope<'a>,
    backend: Backend,
    tenant: Tenant,
) -> v8::Local<'a, v8::ObjectTemplate> {
    let template = v8::ObjectTemplate::new(scope);

    add_trading_api_name_property(scope, &template, backend, tenant);

    return template;
}

fn add_trading_api_name_property<'a>(
    scope: &mut v8::HandleScope<'a>,
    object_template: &v8::Local<'a, v8::ObjectTemplate>,
    backend: Backend,
    tenant: Tenant,
) {
    let key = v8::String::new(scope, "tradingApiName").unwrap().into();
    let getter = v8::FunctionTemplate::new(
        scope,
        |handle_scope: &mut v8::HandleScope,
         _args: v8::FunctionCallbackArguments,
         mut rv: v8::ReturnValue| {
            rv.set(v8::String::new(handle_scope, "alpaca").unwrap().into());
        },
    );

    object_template.set_accessor_property(
        key,
        Some(getter),
        None,
        v8::PropertyAttribute::READ_ONLY,
    );
}
