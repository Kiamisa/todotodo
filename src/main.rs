use std::env;
use todo_todo::{help, Todo};

fn main(){
    let todo = Todo::new().expect("Não foi possível criar o TODO :(");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "mk" => todo.add(&args[2..]),
            "rm" => todo.remove(&args[2..]),
            "done" => todo.done(&args[2..]),
            "raw" => todo.raw(&args[2..]),
            "edit" => todo.edit(&args[2..]),
            "sort" => todo.sort(),
            "reset" => todo.reset(),
            "restore" => todo.restore(),
            "help" | "--help" | "-h" | _ => help(),            
        }
    } else {
        todo.list();
    }
}
