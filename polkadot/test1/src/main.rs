macro_rules! my_macro {
    () => {
        println!("hello from my_macro");
    };
}

fn main() {
    let body = async {
        my_macro!();
    };
    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body);
}

