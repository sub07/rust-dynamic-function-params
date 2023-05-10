use crate::context::ContextDataView;
use crate::dyn_fn::DynFnParam;
use context::ContextData;
use dyn_fn::DynFn;
use std::thread;
use std::time::Duration;

mod context;
mod dyn_fn;

fn take_fn<F, Args: DynFnParam>(f: F, context: &ContextData)
where
    F: DynFn<Args>,
{
    for _ in 0..10 {
        f.call(Args::get_param(context));
        thread::sleep(Duration::from_secs_f32(0.2))
    }
}

fn test(p1: ContextDataView<u8>, p2: ContextDataView<i32>, p3: ContextDataView<&'static str>) {
    println!("test called: {p1:?} {p2:?} {p3:?}");
    *p1.borrow_mut() += 1;
}

fn main() {
    let mut context = ContextData::default();
    context.put(6i32);
    context.put("oufi");
    context.put(9u8);
    take_fn(test, &context);
}
