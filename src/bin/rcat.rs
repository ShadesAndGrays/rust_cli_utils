use std::{fs::File, io::Read};
use clap::Parser;

#[derive(Parser)]
struct Args{
    #[arg(short,long,num_args=0..)]
    file: Vec<String>
}

fn main() {
    let args = Args::parse();
    let mut content = String::new();
    for i in args.file{
        match File::open(&i){
            Ok(mut value) => {
                value.read_to_string(&mut content).unwrap();
                println!("{content}");},
            Err(_) => {
                println!("rcat: {} :No such file or directory",&i);
                continue;
            }

        };

    }
}
