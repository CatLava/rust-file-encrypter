use simple_crypt::{encrypt, encrypt_file, decrypt, decrypt_file};
use std::path::Path;
use std::fs;
use clap::Parser;
use std::fmt::format;


mod cli_options;



fn main() {
    let file_args: cli_options::FileArgs = cli_options::FileArgs::parse();
    // first encrypt a file and see the outpt
    // todo need a check to ensure vec is not null
    let paths = file_args.file_paths;
    let password = file_args.password;
    if file_args.decrypt{
        println!("Attempting to decrypt file");
        for path in paths {
            let decyrpted_file = decrypt_file(Path::new(&path), Path::new(&format!("{}.fixed", path)), password.as_bytes());
            
        }
    } else {
        for path in paths {
            let encyrpted_file = encrypt_file(Path::new(&path), Path::new(&format!("{}.enc", path)), password.as_bytes());
            if file_args.delete_original {
                fs::remove_file(&path).expect("unable to remove file")
            }
        }
    }
}
