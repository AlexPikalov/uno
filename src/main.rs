use std::fs::File;

use cranelift_codegen::{
    ir::{types, AbiParam, ExtFuncData, Function, InstBuilder, UserFuncName},
    isa, settings, Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use lang::ast::expr_parser;

mod compiler;
mod parser;
mod uno_ir;
mod uno_std;
mod lang;

fn main() {
    let code = r#"
    fn main(argc i64, argv i64) i64 {
        return 0
    }
"#;
    let module = expr_parser::statements(code).expect("main: unable to parse code");
    println!("PARSED: \n {module:#?}");
//    let shared_builder = settings::builder();
//    let shared_flags = settings::Flags::new(shared_builder);
//    let target = "x86_64";
//    let isa_builder = isa::lookup_by_name(target).unwrap();
//    let isa = isa_builder.finish(shared_flags).unwrap();
//
//    let obj_builder = ObjectBuilder::new(isa, "main", default_libcall_names())
//        .expect("should create object builder");
//    let mut obj_module = ObjectModule::new(obj_builder);
//
//    let mut sig = obj_module.make_signature();
//    sig.returns.push(AbiParam::new(types::I32));
//    let fn_main = obj_module
//        .declare_function("main", Linkage::Export, &sig)
//        .expect("should declare main function");
//    let namespace = 0;
//    let fn_idx = 0;
//    let mut func = Function::with_name_signature(UserFuncName::user(namespace, fn_idx), sig);
//    let mut fn_ctx = FunctionBuilderContext::new();
//    let mut fn_builder = FunctionBuilder::new(&mut func, &mut fn_ctx);
//    let block = fn_builder.create_block();
//    fn_builder.switch_to_block(block);
//    fn_builder.seal_block(block);
//    let return_val = fn_builder.ins().iconst(types::I32, 12);
//    fn_builder.ins().return_(&[return_val]);
//    let mut ctx = Context::for_function(func);
//    obj_module
//        .define_function(fn_main, &mut ctx)
//        .expect("should define function");
//    let res_obj = obj_module.finish();
//    let mut output = File::create("./hello_world.o").expect("should create object file");
//    res_obj
//        .object
//        .write_stream(&mut output)
//        .expect("should write object file");

    println!("DONE");
}
