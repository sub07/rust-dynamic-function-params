use context::ContextData;
use dyn_fn::DynFn;
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;

mod context;
mod dyn_fn;

fn take_fn<F, Types>(f: F, context: &mut ContextData)
where
    F: DynFn<Types>,
{
    for _ in 0..10 {
        f.call_with_context(context);
        sleep(Duration::from_secs_f32(0.2))
    }
}

fn test(p1: &RefCell<u8>, p2: &RefCell<&'static str>, p3: &RefCell<i32>) {
    println!("test called: {p1:?} {p2:?} {p3:?}");
    *p1.borrow_mut() += 1;
}

fn main() {
    let mut context = ContextData::default();
    context.put(6i32);
    context.put("oufi");
    context.put(9u8);
    take_fn(test, &mut context);
}
