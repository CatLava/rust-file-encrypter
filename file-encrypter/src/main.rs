use simple_crypt::{encrypt, encrypt_file};
use std::path::Path;
use clap::Parser;
use std::fmt::format;

mod cli_options;



fn main() {
    println!("Hello, world!");
    let file_args: cli_options::FileArgs = cli_options::FileArgs::parse();
    // first encrypt a file and see the outpt
    // todo need a check to ensure vec is not null
    let paths = file_args.file_paths;
    for path in paths {
        let encyrpted_file = encrypt_file(Path::new(&path), Path::new(&format!("{}.enc", path)), b"test");
    }
    // Need to verify return type
    // Need to get the args added in here for correct usage 
    //let _ = encrypt_file(Path::new("./test.txt"), Path::new("./encrypted-text.txt"), b"test");
}
