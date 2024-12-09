use ethers::contract::Abigen;
use heck::ToSnakeCase;

fn main() {
    // Only re-run the builder script if the contract changes
    println!("cargo:rerun-if-changed=abi/GNSPairsStorage.json");
    bindgen("GNSPairsStorage");
}

#[allow(dead_code)]
fn bindgen(fname: &str) {
    let bindings = Abigen::new(fname, format!("./abi/{}.json", fname))
        .expect("could not instantiate Abigen")
        .generate()
        .expect("could not generate bindings");

    bindings
        .write_to_file(format!("./src/bindings/{}.rs", fname.to_snake_case()))
        .expect("could not write bindings to file");
}
