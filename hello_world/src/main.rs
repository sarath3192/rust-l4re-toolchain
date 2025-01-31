use base64::prelude::*;

fn main() {
    let hello = "Hello, world! (from Rust)";
    println!("{hello}");
    println!("Now as base64: {}", BASE64_STANDARD.encode(hello));
}
