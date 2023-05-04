use context::Context;
use dyn_fn::DynFn;

mod context;
mod dyn_fn;

fn take_fn<F, Params>(f: F, context: &Context) where F: DynFn<Params> {
    f.call_with_context(context);
}

fn test(i: &&'static str, i2: &i32, i3: &u8) {
    println!("test called: {i} {i2} {i3}");
}

fn main() {
    let mut context = Context::default();
    context.put(6);
    context.put("oufi");
    context.put(9u8);
    take_fn(|a: &&'static str| {println!("{a}")}, &context);
}
