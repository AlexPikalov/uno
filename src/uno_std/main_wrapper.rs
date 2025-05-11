use cranelift_codegen::{
    ir::{types, AbiParam, ExtFuncData, Function, Signature, UserFuncName},
    isa::CallConv,
    Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module};
use cranelift_object::ObjectModule;

pub fn main_wrapper(
    obj_module: &mut ObjectModule,
    namespace: u32,
    index: u32,
    call_conv: CallConv,
) {
    let fn_start_name = UserFuncName::user(namespace, index);
    // _start signature definition
    let mut fn_start_sig = Signature::new(call_conv);
    fn_start_sig
        .params
        .extend_from_slice(&[AbiParam::new(types::I64), AbiParam::new(types::I64)]);
    fn_start_sig.returns.push(AbiParam::new(types::I64));
    // _start declaration
    let fn_start_id = obj_module
        .declare_function("_start", Linkage::Export, &fn_start_sig)
        .expect("unable to declare symbol _start");
    let mut fn_start = Function::with_name_signature(fn_start_name, fn_start_sig);
    // _start body definition
    let mut fn_start_builder_ctx = FunctionBuilderContext::new();
    let mut fn_builder_start = FunctionBuilder::new(&mut fn_start, &mut fn_start_builder_ctx);
    let sig_syscall_exit = fn_builder_start.import_signature(signature);
    let fn_syscall_exit = fn_builder_start.import_function(ExtFuncData{});
    let block = fn_builder_start.create_block();
    fn_builder_start.switch_to_block(block);
    fn_builder_start.seal_block(block);
    // call imported function
    // _start definition in object module
    let mut ctx_fn_main = Context::new();
    obj_module
        .define_function(fn_start_id, &mut ctx_fn_main)
        .expect("unable to define function _start in object module");
}
