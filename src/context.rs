use std::any::{Any, TypeId};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct Context {
    resources: HashMap<TypeId, Rc<dyn Any>>,
}

impl Context {
    pub fn get<T: Any>(&self) -> Rc<RefCell<Box<T>>> {
        self.resources.get(&TypeId::of::<T>()).unwrap().clone().downcast().unwrap()
    }

    pub fn put<T: Any>(&mut self, val: T) {
        self.resources.insert(val.type_id(), Rc::new(RefCell::new(Box::new(val))));
    }
}
