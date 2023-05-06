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

    // pub fn borrow<T1: Any, T2: Any, T3: Any, F>(&mut self, f: F) where F: FnOnce(&mut T1, &mut T2, &mut T3) {
    //     let mut b1 = self.get::<T1>();
    //     let mut b2 = self.get::<T2>();
    //     let mut b3 = self.get::<T3>();
    //
    //     f(b1.as_mut(), b2.as_mut(), b3.as_mut());
    //
    //     self.put_back(b1);
    //     self.put_back(b2);
    //     self.put_back(b3);
    // }

    pub fn put<T: Any>(&mut self, val: T) {
        self.resources
            .insert(val.type_id(), Box::new(RefCell::new(val)));
    }
}
