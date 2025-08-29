use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{self, BufReader, BufWriter, Writer};
use std::path::Path;
use std::{env, process};

pub struct Entry{
    pub todo_entry: String,
    pub done: bool,
}

impl Entry{
    pub fn new(todo_entry: String, done: bool) -> self{
        Self{
            todo_entry,
            done,
        }
    }

    pub fn file_line(&self) -> String{
        let symbol = if self.done { "[*] " } else { "[ ] " };
        format!("{}{}\n", symbol, self.todo_entry,)
    }

    pub fn list_list(&self, number: usize) -> String{
        let todo_entry = if self.done{
            self.todo_entry.strikethrough().to_string()
            }else{
                self.todo_entry.clone()
        };
        format!("{number} {todo_entry}\n")
    }

    pub fn read_line(line: &String) -> Self{
        let done = &line[..4] == "[*] ";
        let todo_entry = (&line[4..]).to_string();
        Self{
            todo_entry,
            done,
        }
    }

    pub fn raw_line(&self) -> String{
        format!("{}\n", self.todo_entry)
    }
}

pub struct Todo{
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

impl Todo{
    pub fn new() -> Result<Self, String>{
        let todo_path: String = match env::var("TODO_PATH"){
            Ok(t) => t,
            Err(_) =>{
                let home = env::var("HOME").unwrap();


                let legacy_todo = format!("{}/TODO", &home);
                match Path::new(&legacy_todo).exists(){
                    true => legacy_todo,
                    false => format!("{}/.todo", &home),
                }
                
            }
        };
    
        let todo_bak: String = match env::var("TODO_BAK_DIR"){
            Ok(t) => t,
            Err(_) => String::from("/tmp/todo.bak")
        };

        let no_backup = env::var("TODO_BACKUP").is_ok();
        
        let todofile = OpenOptions::new()
                        .write(true)
                        .read(true)
                        .create(true)
                        .open(&todo_path)
                        .expect("Não foi possivel abrir o arquivo");

        let mut buf_reader = BufReader::new(&todofile);

        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let todo = contents.lines().map(str::to_string).colletc();

        Ok(Self{
            todo,
            todo_path,
            todo_bak,
            no_backup,
        })
    }

}
