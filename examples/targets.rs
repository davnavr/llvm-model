/// Test to check that target triple and layout information is correct.
///
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    use llvm_model::{interop, target};

    unsafe {
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetInfos();
        llvm_sys::target::LLVM_InitializeNativeTarget();

        let host_machine = interop::llvm_sys::target::TargetMachine::host_machine(
            target::CodeGenerationOptimization::Default,
            target::RelocationMode::Default,
            target::CodeModel::Default,
        ).unwrap();

        println!(
            "Current: {:?}",
            &host_machine,
        );

        println!("Current Layout {:?}", interop::llvm_sys::target::TargetLayout::try_from(&host_machine));
    }
}
