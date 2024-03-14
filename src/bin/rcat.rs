use std::{fs::File, io::Read};

use clap::Parser;

#[derive(Parser)]
struct Args{
    #[arg(short,long)]
    file : String
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.file).unwrap();
    let mut content = String::new();
    
    file.read_to_string(&mut content).unwrap();

    println!("{content}");
}
