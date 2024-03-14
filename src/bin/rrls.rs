use core::panic;
use std::{env::current_dir, path::Path};

use clap::Parser;

#[derive(Parser)]
struct Args{
    #[arg(short,long,default_value=".")]
    path:String

}
fn main(){
    let args = Args::parse();

    let  cd = current_dir().unwrap();
    let mut new_path = cd.join(Path::new(&args.path)); 

    if !new_path.try_exists().unwrap(){
        panic!("Path does not exist");
    }


    if !new_path.is_dir(){
        panic!("Path is not a dir");
    }


    new_path = new_path.canonicalize().unwrap(); // Incase of .. or . being in the path like
                                                 // /home/../some/path/somewhere

    println!("Path :{}",new_path.to_str().unwrap()); 
    let paths:Vec<Result<std::fs::DirEntry, std::io::Error>> = new_path.read_dir().unwrap().collect();
    for i in paths{
      println!("{}",i.unwrap().path().to_str().unwrap());
    }
}
