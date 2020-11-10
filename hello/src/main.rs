use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::FunctionType;
use inkwell::{OptimizationLevel, AddressSpace};
use inkwell::module::Linkage;

// hello world
// https://github.com/TheDan64/inkwell/issues/32
fn main() {
    let context = Context::create();
    let module = context.create_module("repl");

    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let function_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

    let function = module.add_function("main", function_type, None);
    let basic_block = context.append_basic_block(function, "entrypoint");

    builder.position_at_end(basic_block);

    builder.build_global_string_ptr("hello, world!\n", "hello");

    let i32_type = context.i32_type();
    let str_type = context.i8_type().ptr_type(AddressSpace::Generic);
    let printf_type = i32_type.fn_type(&[str_type.into()], true);

    let printf = module.add_function("printf", printf_type, Some(Linkage::External));
    // builder.build_call(&printf, &[global], "");

    // builder.build_return(Some(&i32_type.const_int(0, false)));


    println!("Hello, world!");
}
