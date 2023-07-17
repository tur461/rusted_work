use wasmtime::{Config, Engine, Instance, Module, Store, Val};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config
        .wasm_threads(true)
        .wasm_reference_types(true)
        .wasm_simd(true);

    // Create an instance of the Wasmtime engine
    let engine = Engine::new(&config).unwrap();

    // Create a store to hold the runtime state
    let store = Store::new(&engine);

    // Load the Wasm module from a binary file
    let module = Module::from_file(&store, "./wasm_module.wasm")?;

    // Instantiate the Wasm module
    let instance = Instance::new(&store, &module, &[])?;

    // Access exported functions from the Wasm module
    let add_fn = instance.get_func("add")?.get0::<i32, i32>()?;

    // Call the exported function with arguments and get the result
    let result = add_fn.call(2)?;

    println!("Result: {}", result);

    Ok(())
}
