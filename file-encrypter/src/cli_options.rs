

// option for phrase, file, or dir
// also need output name if optional
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // File to encrypt or decrypt, eventually make this an array
    #[arg(short, long)]
    file_paths: Vec[String],

    // Password to encrypt file with
    #[arg(short, long)]
    password: String,

    //decrypt option 
    #[arg(short, long, default_value_t = False)]
    decrypt: bool
}