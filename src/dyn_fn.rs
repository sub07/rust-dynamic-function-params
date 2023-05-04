
use crate::context::Context;

pub trait DynFn<Params> {
    fn call_with_context(&self, context: &Context);
}

macro_rules! make_dyn_fn {
    ($($type_gen:ident)*) => {
        impl <F, $($type_gen: std::any::Any),*> DynFn<($(&$type_gen,)*)> for F where F: Fn($(&$type_gen,)*) {
            #![allow(unused_variables)]
            fn call_with_context(&self, context: &crate::Context) {
                self($(&**context.get::<$type_gen>().borrow(),)*);
            }
        }
    };
}

make_dyn_fn!();
make_dyn_fn!(P1);
make_dyn_fn!(P1 P2);
make_dyn_fn!(P1 P2 P3);
make_dyn_fn!(P1 P2 P3 P4);
make_dyn_fn!(P1 P2 P3 P4 P5);
make_dyn_fn!(P1 P2 P3 P4 P5 P6);
make_dyn_fn!(P1 P2 P3 P4 P5 P6 P7);
make_dyn_fn!(P1 P2 P3 P4 P5 P6 P7 P8);
make_dyn_fn!(P1 P2 P3 P4 P5 P6 P7 P8 P9);
make_dyn_fn!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10);
