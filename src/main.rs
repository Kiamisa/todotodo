use std::env;
use todo_bin::{help, Todo};

fn main(){
    let todo = Todo::new().expect("Não foi possível criar o TODO :(");

    let args: Vec<String> = env::args().collect();

    
}