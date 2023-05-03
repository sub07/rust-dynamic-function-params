use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

trait VarFn<Params> {
    fn call(&self, param: &Params);
}

macro_rules! make_var_fn {
    ($($type_gen:ident)*) => {
        impl <F, $($type_gen),*> VarFn<($($type_gen,)*)> for F where F: Fn($(&$type_gen,)*) {
            #[allow(non_snake_case)]
            fn call(&self, ($($type_gen,)*): &($($type_gen,)*)) {
                self($($type_gen,)*);
            }
        }
    };
}

make_var_fn!();
make_var_fn!(P1);
make_var_fn!(P1 P2);
make_var_fn!(P1 P2 P3);
make_var_fn!(P1 P2 P3 P4);
make_var_fn!(P1 P2 P3 P4 P5);
make_var_fn!(P1 P2 P3 P4 P5 P6);
make_var_fn!(P1 P2 P3 P4 P5 P6 P7);
make_var_fn!(P1 P2 P3 P4 P5 P6 P7 P8);
make_var_fn!(P1 P2 P3 P4 P5 P6 P7 P8 P9);
make_var_fn!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10);

#[derive(Default)]
pub struct Context {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn put<T: Any>(&mut self, val: T) {
        self.resources.insert(val.type_id(), Box::new(val));
    }
}

struct Res<T: Any> {
    //make trait
    val: RefCell<T>,
}

struct ResMut<T: Any> {
    val: RefCell<T>,
}

trait ContextDestruct<'a>: Sized {
    fn destruct(context: &'a Context) -> Option<Self>;
}

impl<'a, P1: Any> ContextDestruct<'a> for (&'a P1, ) {
    fn destruct(context: &'a Context) -> Option<Self> {
        Some((context.get()?, ))
    }
}

impl<'a, P1: Any, P2: Any> ContextDestruct<'a> for (&'a P1, &'a P2) {
    fn destruct(context: &'a Context) -> Option<Self> {
        Some(
            (
                context.get()?,
                context.get()?,
            )
        )
    }
}

fn take_fn<'a, F, Params: 'static + ContextDestruct<'a>>(context: &'a Context, f: F) where F: VarFn<Params> {
    let p = Params::destruct(context);
}

fn test(i: &i32, s: &&'static str) {
    println!("test called: {i} {s}");
}

fn main() {
    let mut context = Context::default();
    context.put(5);
    context.put("oui");
    context.put(9u8);
    take_fn(&context, test);
}
