use simple_crypt::{encrypt, encrypt_file};
use std::path::Path;
use clap::Parser;

mod cli_options::Args;



fn main() {
    println!("Hello, world!");
    let file_args = Args::parse()
    // first encrypt a file and see the outpt
    // Need to verify return type
    // Need to get the args added in here for correct usage 
    let _ = encrypt_file(Path::new("./test.txt"), Path::new("./encrypted-text.txt"), b"test");
}
