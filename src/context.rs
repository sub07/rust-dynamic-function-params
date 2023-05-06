use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct ContextData {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl ContextData {
    pub fn get<T: Any>(&mut self) -> Box<RefCell<T>> {
        self.resources
            .remove(&TypeId::of::<T>())
            .expect("No parameter of this type in context data")
            .downcast()
            .unwrap()
    }

    pub fn put_back<T: Any>(&mut self, val: Box<RefCell<T>>) {
        self.resources.insert(TypeId::of::<T>(), val);
    }

    pub fn put<T: Any>(&mut self, val: T) {
        self.resources
            .insert(val.type_id(), Box::new(RefCell::new(val)));
    }
}
