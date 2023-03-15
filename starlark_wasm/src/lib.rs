use std::convert::TryInto;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn do_stuff() -> Result<(), JsValue> {
    log("Hello from Rust!");

    log("Cheese 2.0");

    // Create an environment and module for evaluation
    // let global_env = Environment::new("global");
    let module = Module::new();

    // Define the Starlark expression
    let expression = "3 + 3";

    // Parse the expression
    let ast =
        AstModule::parse("test_expression", expression.to_owned(), &Dialect::Standard).unwrap();
    log("ast parsed");

    // Create an evaluator
    let mut evaluator = Evaluator::new(&module);
    log("evaluator created");

    let globals = Globals::new();
    log("globals created");

    // Call eval_module which has signature     pub fn eval_module(&mut self, ast: AstModule, globals: &Globals) -> anyhow::Result<Value<'v>> {
    let result = evaluator.eval_module(ast, &globals).unwrap();
    // log(s)


    // log result
    log(&format!("Result: {}", result.to_repr()));

    // eval.eval_module(module, &globals)?;

    return Ok(());
}
