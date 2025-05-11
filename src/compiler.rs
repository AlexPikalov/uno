use std::sync::Arc;

use cranelift_codegen::{
    ir::{types, AbiParam, Function, InstBuilder, Signature, UserFuncName, Value},
    isa,
    settings::{self, Configurable},
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

pub struct Compiler {
    obj_module: ObjectModule,
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut shared_builder = settings::builder();
        let shared_flags = settings::Flags::new(shared_builder);
        let target = "x86_64";
        let isa_builder = isa::lookup_by_name(target).unwrap();
        let isa = isa_builder.finish(shared_flags).unwrap();

        let obj_builder = ObjectBuilder::new(isa, "main", default_libcall_names())
            .expect("should create object builder");
        let mut obj_module = ObjectModule::new(obj_builder);
        let mut module_ctx = obj_module.make_context();

        let mut sig = obj_module.make_signature();
        sig.returns.push(AbiParam::new(types::I32));
        let fn_main = obj_module
            .declare_function("main", Linkage::Export, &sig)
            .expect("should declare main function");
        obj_module.define_function(fn_main, &mut module_ctx).expect("should define function");
        let namespace = 0;
        let fn_idx = 0;
        let mut func = Function::with_name_signature(UserFuncName::user(namespace, fn_idx), sig);
        let mut fn_ctx = FunctionBuilderContext::new();
        let mut fn_builder = FunctionBuilder::new(&mut func, &mut fn_ctx);
        fn_builder.ins().return_(&[Value::from_u32(0)]);
        Compiler { obj_module }
    }
}
