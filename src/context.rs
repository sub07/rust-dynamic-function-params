use crate::dyn_fn::DynFnParam;
use rust_utils_macro::New;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Default)]
pub struct ContextData {
    resources: HashMap<TypeId, Rc<dyn Any>>,
}

impl ContextData {
    pub fn get<T: Any>(&self) -> Rc<RefCell<T>> {
        self.resources
            .get(&TypeId::of::<T>())
            .expect("No data of this type in context")
            .clone()
            .downcast()
            .unwrap()
    }

    pub fn put<T: Any>(&mut self, val: T) {
        self.resources
            .insert(val.type_id(), Rc::new(RefCell::new(val)));
    }
}

#[derive(New, Debug)]
pub struct ContextDataView<T> {
    inner: Rc<RefCell<T>>,
}

impl<T: Any> Deref for ContextDataView<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<T: Any> DerefMut for ContextDataView<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.inner).unwrap()
    }
}

impl<T: Any> DynFnParam for ContextDataView<T> {
    fn get_param(context: &ContextData) -> Self {
        ContextDataView::new(context.get())
    }
}
