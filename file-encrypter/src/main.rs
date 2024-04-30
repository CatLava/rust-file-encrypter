use simple_crypt::{encrypt, encrypt_file};
use std::path::Path;



fn main() {
    println!("Hello, world!");
    // first encrypt a file and see the outpt
    let _ = encrypt_file(Path::new("./test.txt"), Path::new("./encrypted-text.txt"), b"test");
}
