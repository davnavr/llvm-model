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

    for triple in example_triples {
        let target_triple = triple.to_inkwell_target();

        println!("{:?}", target_triple);
    }
}
