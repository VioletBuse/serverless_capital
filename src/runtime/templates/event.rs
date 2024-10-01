use v8::ObjectTemplate;

use crate::{backend::Backend, tenant::Tenant};

#[derive(Debug, Clone)]
pub enum Event {
    YOLO,
}

impl Event {
    pub fn create_event_object_template<'a>(
        self,
        scope: &mut v8::HandleScope<'a>,
        backend: Backend,
        tenant: Tenant,
    ) -> v8::Local<'a, ObjectTemplate> {
        let template = v8::ObjectTemplate::new(scope);

        return template;
    }
}
