use deno_core::JsRuntime;
use deno_core::RuntimeOptions;


pub fn main() {
    let mut runtime = JsRuntime::new(RuntimeOptions {
    ..Default::default()
  });

  runtime.execute_script(
    "<usage>",
    include_str!("temp.js")
  ).unwrap();
    
}
