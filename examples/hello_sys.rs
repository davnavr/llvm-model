/// Test to showcase basic APIs of `llvm-model` as well as interoperation with `llvm-sys`.
fn main() {
    use llvm_model::{global, interop, target, types, Identifier};

    // Gathers information about the target machine, unsafe as it calls LLVM C API functions.
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

    let mut builder =
        interop::llvm_sys::ModuleBuilder::new(Identifier::try_from("hello").unwrap(), &host_target);

    {
        let module = builder.module();

        module.add_global_value(global::Function::new(
            Identifier::try_from("main").unwrap(),
            types::Function::new(types::Return::Void, Vec::new()),
        ));

        println!("{}", module);
    }

    println!(
        "{}",
        unsafe { builder.into_message(llvm_sys::core::LLVMGetGlobalContext()) }
            .unwrap()
            .to_identifier()
    );
}
