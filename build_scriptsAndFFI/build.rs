fn main() {
    println!("cargo:warning=Welcome from build script!!");
    println!("cargo:warning=Generating foo.rs in OUT_DIR");
    std::fs::write(
        std::path::Path::new(
            &std::env::var("OUT_DIR").unwrap()
        ).join("foo.rs"), 
        r"pub fn foo() { dbg!(); }"
    ).unwrap();
}
