/// Test to check that target triple and layout information is correct.
/// 
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    // Safety: Performs initialization of LLVM target information.
    unsafe {
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetInfos();
        assert_ne!(llvm_sys::target::LLVM_InitializeNativeTarget(), 1);
    }
    
    println!("hello")
}
