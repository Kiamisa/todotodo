use std::env;
mod cli;
mod gui;
mod libs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.contains(&"--gui".to_string()) {
        let todo = libs::Todo::new().expect("Não foi possível criar o TODO :(");
        gui::app::run(todo);
    } else {
        cli::run(args);
    }
}
