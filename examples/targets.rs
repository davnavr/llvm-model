/// Test to check that target triple and layout information is correct.
///
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    use llvm_model::{interop, target, Identifier};

    let context = unsafe { llvm_sys::core::LLVMGetGlobalContext() };

    let host_target = unsafe {
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetInfos();
        llvm_sys::target::LLVM_InitializeNativeTarget();

        interop::llvm_sys::target::Target::host_machine_target(
            target::CodeGenerationOptimization::Default,
            target::RelocationMode::Default,
            target::CodeModel::Default,
        )
        .unwrap()
    };

    let module = interop::llvm_sys::ModuleBuilder::new(
        Identifier::try_from("target_test").unwrap(),
        &host_target,
    );

    println!("{}", unsafe { module.into_message(context) }.unwrap().to_identifier());
}
