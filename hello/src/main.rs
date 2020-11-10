use inkwell::context::Context;
use inkwell::{OptimizationLevel, AddressSpace};
use inkwell::module::Linkage;
use inkwell::execution_engine::{FunctionLookupError, JitFunction};

// hello world
// https://github.com/TheDan64/inkwell/issues/32
fn main() {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let function_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

    let function = module.add_function("main", function_type, None);
    let basic_block = context.append_basic_block(function, "entrypoint");

    builder.position_at_end(basic_block);

    let i32_type = context.i32_type();
    let str_type = context.i8_type().ptr_type(AddressSpace::Generic);
    let printf_type = i32_type.fn_type(&[str_type.into()], true);

    let printf = module.add_function("printf", printf_type, Some(Linkage::External));

    let string = "hello, world!\n";
    let name = "hello";

    let ty = context.i8_type().array_type(string.len() as u32);
    let gv = module.add_global(ty, Some(AddressSpace::Generic), name);
    gv.set_linkage(Linkage::Internal);
    gv.set_initializer(&context.const_string(string.as_ref(), false));

    let pointer_value = builder.build_pointer_cast(
        gv.as_pointer_value(),
        context.i8_type().ptr_type(AddressSpace::Generic),
        name,
    );
    builder.build_call(printf, &[pointer_value.into()], "");

    builder.build_return(Some(&i32_type.const_int(0, false)));

    let ee = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let maybe_fn = unsafe {
        ee.get_function::<unsafe extern "C" fn() -> f64>("main")
    };

    let compiled_fn = match maybe_fn {
        Ok(f) => f,
        Err(_) => {
            panic!();
        }
    };

    unsafe  {
        compiled_fn.call();
    }
}
