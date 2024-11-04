use ethers::contract::Abigen;
use heck::ToSnakeCase;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=abi");
    generate("EventEmitter");
}

fn generate(name: &str) {
    Abigen::new(name, format!("abi/{}.json", name))
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(format!("src/bindings/{}.rs", name.to_snake_case()))
        .unwrap();
}
