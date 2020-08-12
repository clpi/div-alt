use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let now = std::time::Instant::now();
    let mut num: i32 = 0;
    for i in 1..10000 { num += i; }
    let after = std::time::Instant::now();
    let dur = after.duration_since(now).as_secs_f64();
    Ok(cx.string(format!("{} seconds: Hello, from rust! {}", dur, num)))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
