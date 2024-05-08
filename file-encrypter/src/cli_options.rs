use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FileArgs {
    // File to encrypt or decrypt, eventually make this an array
    #[arg(short, long)]
    pub file_paths: Vec<String>,

    // Password to encrypt file with
    #[arg(short, long)]
    pub password: String,

    //decrypt option 
    #[arg(short, long, default_value_t = false)]
    pub decrypt: bool,

    //delete original file; this would simulate a malware, where files become encrypted 
    #[arg(long, default_value_t = false)]
    pub delete_original: bool
}