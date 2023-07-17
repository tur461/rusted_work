#[allow(unused_imports)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

// const STORAGE_FILE_PATH: &str = "./recipes.json";
const WASM_PATH: &str = "./wasm/pkg/wasm_bg.wasm";
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
// static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
// static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()))
// static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"))

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");
    let op = Command::new("sh")
    .arg("-c")
    .arg("pwd")
    .output()
    .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&op.stdout));

    let mut buffer = Vec::new();
    {
        let mut data = File::open(WASM_PATH)?;
        data.read_to_end(&mut buffer)?;
    }
    let module = wasmi::Module::from_buffer(buffer)?;
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
                    .expect("Failed to instantiate WASM module")
                    .assert_no_start();
    let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(17));
    args.push(RuntimeValue::from(18));
    let result: Option<RuntimeValue> = instance.invoke_export("add", &args, &mut NopExternals)?;
    match result {
        Some(RuntimeValue::I32(val)) => {
            println!("The answer to your sum was {}", val);
        }
        Some(_) => {
            println!("Got a value of an unexpected data type");
        }
        None => {
            println!("Failed to get a result from wasm invocation");
        }
    }
    Ok(())
}
