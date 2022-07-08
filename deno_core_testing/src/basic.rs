use deno_core::JsRuntime;
use deno_core::RuntimeOptions;


pub fn main() {
    let mut runtime = JsRuntime::new(RuntimeOptions {
    ..Default::default()
  });

  runtime.execute_script(
    "<usage>",
    r#"
    const arr = [1,2,3]
    Deno.core.print(arr.toString())
    "#
  ).unwrap();
    
}
