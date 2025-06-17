use std::sync::OnceLock;

use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::Value;
use starlark::values::none::NoneType;
use starlark::environment::{Globals, GlobalsBuilder, Module};

fn globals() -> &'static Globals {
    static GLOBALS: OnceLock<Globals> = OnceLock::new();
    GLOBALS.get_or_init(|| GlobalsBuilder::new().with(global).build())
}

#[starlark::starlark_module]
fn global(builder: &mut GlobalsBuilder) {
    fn print(x: Value) -> starlark::Result<NoneType> {
        tracing::info!("{:?}", x);
        Ok(NoneType)
    }
}

pub fn post_request(script: String, status_code: usize) {
    let ast = AstModule::parse(
        "post_request.star",
        script,
        &Dialect::default(),
    ).unwrap();

    let module = Module::new();
    let heap = module.heap();

    module.set("status_code", heap.alloc(status_code));

    let mut eval = Evaluator::new(&module);
    eval.eval_module(ast, globals()).unwrap();
}
