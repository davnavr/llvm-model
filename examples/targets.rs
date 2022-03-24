/// Test to check that target triple and layout information is correct.
///
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    unsafe {
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetInfos();

        println!(
            "Current: {:?}",
            llvm_model::interop::llvm_sys::target::host_target_triple()
        );
    }
}
