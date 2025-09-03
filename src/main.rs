use std::env;
mod cli;
mod gui;
mod libs;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.contains(&"--gui".to_string()){
        println!("GUI ainda não implementada, usando CLI");
        cli::run(args);
    } else {
        cli::run(args);
    }

}
