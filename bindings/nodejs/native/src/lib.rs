
use neon::prelude::*;
 

fn greet(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(multilib_example::greet()))
}


// Export the class
register_module!(mut m, {
    m.export_function("greet", greet)?;
    Ok(())
});