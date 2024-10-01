use v8::ObjectTemplate;

use crate::{backend::Backend, tenant::Tenant};

pub fn create_storage_api_template<'a>(
    scope: &mut v8::HandleScope<'a>,
    backend: Backend,
    tenant: Tenant,
) -> v8::Local<'a, ObjectTemplate> {
    let template = v8::ObjectTemplate::new(scope);

    return template;
}
