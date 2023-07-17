
mod foo {
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
}

fn main() {
    println!("build scripts out dir {}", env!("OUT_DIR"));
    foo::foo();
}
