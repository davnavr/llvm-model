/// Test to check that target triple and layout information is correct.
///
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    use llvm_model::{interop, target, Identifier};

    let host_target = unsafe {
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetInfos();
        llvm_sys::target::LLVM_InitializeNativeTarget();

        interop::llvm_sys::target::host_machine_target(
            target::CodeGenerationOptimization::Default,
            target::RelocationMode::Default,
            target::CodeModel::Default,
        )
        .unwrap()
    };

    let module = llvm_model::Module::new(
        Identifier::try_from("target_example").unwrap(),
        &host_target,
    );

    println!("{}", &module);
}
