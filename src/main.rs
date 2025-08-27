use std::env;
use todo_bin::{help, Todo};

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
        }

    } else {
        todo.list();
    }
}
