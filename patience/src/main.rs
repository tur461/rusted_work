#![allow(dead_code)]

use std::future::Future;
use futures::executor::block_on;

fn main() {
    block_on(async_main());
}

async fn async_main() {
    println!("Hello, world!");
    let x = foo1().await;
    let y = foo2().await;
    println!("x = {:?}\ny = {:?}", x, y);
}

async fn foo1() -> usize {
    0
}

fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}


