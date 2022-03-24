/// Test to check that target triple and layout information is correct.
///
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    use llvm_model::target;

    unsafe {
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetInfos();

        println!(
            "Current: {:?}",
            target::Machine::host_machine(
                target::CodeGenerationOptimization::Default,
                target::RelocationMode::Default,
                target::CodeModel::Default
            )
        );
    }
}
