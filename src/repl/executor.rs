use deno_core::{error::AnyError, serde_json, serde_v8, v8, JsRuntime};

pub fn execute_js(runtime: &mut JsRuntime, code: &str) -> Result<String, AnyError> {
    // execute the js code
    let result = runtime.execute_script("<repl>", code.to_string())?;
    // Create a handle scope for V8
    let scope = &mut runtime.handle_scope();

    // convert the result to local reference

    let local = v8::Local::new(scope, result);
    
    // deserliaze
    let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local)?;
    
    // Format the output based on the type of the deserialized value
    match deserialized_value {
        serde_json::Value::Null => Ok("null".to_string()),
        serde_json::Value::Bool(b) => Ok(b.to_string()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::String(s) => Ok(format!("\"{}\"", s)),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            Ok(serde_json::to_string_pretty(&deserialized_value)?)
        }
    }
    
}
