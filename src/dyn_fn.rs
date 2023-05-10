use crate::context::ContextData;

pub trait DynFnParam {
    fn get_param(context: &ContextData) -> Self;
}

macro_rules! impl_tuple_dyn_fn_params {
    ($($generics:ident)*) => {
        impl <$($generics: DynFnParam),*> DynFnParam for ($($generics,)*) {
            #[allow(unused_variables, clippy::unused_unit)]
            fn get_param(context: &ContextData) -> Self {
                ($($generics::get_param(context),)*)
            }
        }
    };
}

impl_tuple_dyn_fn_params!();
impl_tuple_dyn_fn_params!(T1);
impl_tuple_dyn_fn_params!(T1 T2);
impl_tuple_dyn_fn_params!(T1 T2 T3);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4 T5);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4 T5 T6);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4 T5 T6 T7);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4 T5 T6 T7 T8);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_tuple_dyn_fn_params!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);

pub trait DynFn<Args: DynFnParam> {
    fn call(&self, args: Args);
}

macro_rules! impl_dyn_fn {
    ($($generics:ident)*) => {
        impl <F, $($generics: DynFnParam),*> DynFn<($($generics,)*)> for F
        where
            F: Fn($($generics),*),
        {
            #[allow(non_snake_case)]
            fn call(&self, ($($generics,)*): ($($generics,)*)) {
                self($($generics),*);
            }
        }
    };
}

impl_dyn_fn!();
impl_dyn_fn!(P1);
impl_dyn_fn!(P1 P2);
impl_dyn_fn!(P1 P2 P3);
impl_dyn_fn!(P1 P2 P3 P4);
impl_dyn_fn!(P1 P2 P3 P4 P5);
impl_dyn_fn!(P1 P2 P3 P4 P5 P6);
impl_dyn_fn!(P1 P2 P3 P4 P5 P6 P7);
impl_dyn_fn!(P1 P2 P3 P4 P5 P6 P7 P8);
impl_dyn_fn!(P1 P2 P3 P4 P5 P6 P7 P8 P9);
impl_dyn_fn!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10);
