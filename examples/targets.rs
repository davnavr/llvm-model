/// Test to check that target triple and layout information is correct.
///
/// An example is used instead of test functions since LLVM initialization functions probably shouldn't be called more than once.
fn main() {
    use llvm_model::target::{
        Architecture, Environment, KnownTriple, OperatingSystem, Triple, Vendor,
    };

    inkwell::targets::Target::initialize_all(&inkwell::targets::InitializationConfig::default());

    let example_triples = [Triple::from(KnownTriple::with_environment(
        Architecture::X86_64,
        Vendor::PC,
        OperatingSystem::Windows,
        Environment::MSVC,
    ))];

    println!(
        "Host: {}, {}",
        inkwell::targets::TargetMachine::get_default_triple(),
        inkwell::targets::TargetMachine::get_host_cpu_name()
    );

    for triple in example_triples {
        let target_triple = triple.to_inkwell_target().unwrap();

        println!("{:?}", target_triple);
    }
}
