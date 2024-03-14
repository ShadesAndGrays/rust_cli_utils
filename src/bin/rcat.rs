use std::{fs::File, io::Read};
use clap::Parser;

#[derive(Parser)]
struct Args{
    #[arg(num_args=0..)]
    file: Vec<String>
}

fn main() {
    let args = Args::parse();
    let mut content = String::new();
    for i in args.file{
        content.clear();
        match File::open(&i){
            Ok(mut value) => {
                value.read_to_string(&mut content).unwrap();
                println!("{}",content.trim());
            },
            Err(_) => {
                println!("rcat: {} :No such file or directory",&i);
                continue;
            }

        };

    }
}
