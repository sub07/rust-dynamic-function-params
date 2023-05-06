use std::any::Any;
use std::cell::RefCell;

use crate::context::ContextData;

pub trait DynFn<Types> {
    fn call_with_context(&self, context: &mut ContextData);
}

macro_rules! make_dyn_fn {
    ($($type_gen:ident)*) => {
        paste::paste! {
            impl<F, $($type_gen),*> DynFn<($($type_gen,)*)> for F
            where
                F: Fn($(&RefCell<$type_gen>),*),
                $($type_gen: Any),*
            {
                #![allow(unused_variables, non_snake_case)]
                fn call_with_context(&self, context: &mut crate::ContextData) {
                    $(let [<box_ $type_gen>] = context.get();)*
                    self($([<box_ $type_gen>].as_ref()),*);
                    $(context.put_back([<box_ $type_gen>]);)*
                }
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
