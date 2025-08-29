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

    pub fn list(&self){
        let stdout = io::stdout();

        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();

        for(number, task) in self.todo.iter().enumerate(){
            let entry = Entry::read_line(task);

            let number = number + 1;

            let line = entry.list_line(number);
            data.push_str(&line);
        }
        writer.write_all(data.as_bytes()).expect("Falha ao gravação");
    }

    pub fn raw(&self, arg: &[String]){
        if arg.len() > 1 {
            eprintln!("Esse todo só precisa de 1 argumento, não {}", arg.len())
        } else if arg.is_empty(){
            eprintln!("Esse todo precisa de ao menos 1 argumento (done/todo)")
        } else {
            let stdout = io::stdout();
            let mut writer = BufWriter::new(stdout);
            let mut data = String::new();
            let arg = &arg[0];

            for task in self.todo.iter() {
                let entry = Entry::read_line(task);
                if entry.done && arg == "done"{
                    data = entry.raw_line();
                } else if !entry.done && arg == "todo"{
                    data = entry.raw_line();
                }
            writer.write_all(data.as_bytes()).expect("Falha ao gravação");
                }
            }
        }
    
    pub fn add(&self, args: &[String]){
        if args.is_empty(){
            eprintln!("Precisa colocar alguma coisa");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&self.todo_path)
                        .expect("Não foi possível abrir o arquivo");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty(){
                continue;
            }

            let entry = Entry::new(arg.to_string(), false);
            let line = entry.file_line();
            buffer.write_all(line.as_bytes()).expect("Falha ao gravação");
        }
    }

    pub fn remove(&self, args: &[String]){
        if args.is_empty(){
            eprintln!("Precisa remover alguma coisa");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(&self.todo_path)
                        .expect("Não foi possível abrir o arquivo");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos+1).to_string()){
                continue;
            }

            let line = format!("{}\n", line);
            buffer.write_all(line.as_bytes()).expect("Falha ao gravação");
        }
    }

    fn remove_file(&self){
        match fs::remove_file(&self.todo_path){
            Ok(_) => {}
            Err(e) => {
                eprintln!("Erro ao apagar arquivo: {}", e)
            } 
        };
    }

    pub fn reset(&self){
        if !self.no_backup{
            match fs::copy(&self.todo_path, &self.todo_bak){
                Ok(_) => self.remove_file(),
                Err(_) => {
                    eprintln!("Não foi possível fazer backup do arquivo :(")
                }
            }
        } else {
            self.remove_file();
        }
    }

    pub fn restore(&self){
        fs::copy(&self.todo_bak, &self.todo_path).expect("Não foi possível recuperar o arquivo");
    }

    pub fn sort(&self){
        
    }

    pub fn done(&self, args: &[String]){

    }

    pub fn edit(&self, args: &[String]){
        if args.is_empty() || args.len() != 2{
            eprintln!("A edição precisa de 2 argumentos");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(&self.todo_path)
                        .expect("Não foi possível abrir o arquivo");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            let line = if args[0] == (pos+1).to_string(){
                let mut entry = Entry::read_line(line);
                entry.todo_entry = args[1].clone();
                entry.file_line()
            } else {
                format!("{}\n", line);
            };
            buffer.write_all(line.as_bytes()).expect("Falha ao gravação");
        }
    }
}

const TODO_HELP: &str = "Uso: todo [COMANDO] [ARGUMENTOS]
Todo é um organizador de tarefas super rápido e simples escrito em Rust

Exemplo: todo list

Comandos disponíveis:
    - add [TAREFA/S]
        adiciona nova(s) tarefa(s)
        Exemplo: todo add \"comprar cenouras\"

    - edit [ÍNDICE] [TAREFA/S EDITADA/S]  
        edita uma tarefa existente  
        Exemplo: todo edit 1 banana  

    - list  
        lista todas as tarefas  
        Exemplo: todo list  

    - done [ÍNDICE]  
        marca tarefa(s) como concluída(s)  
        Exemplo: todo done 2 3 (marca a segunda e a terceira tarefas como concluídas)  

    - rm [ÍNDICE]  
        remove uma tarefa  
        Exemplo: todo rm 4  

    - reset  
        apaga todas as tarefas  

    - restore  
        restaura o backup mais recente após um reset  

    - sort  
        organiza tarefas concluídas e não concluídas  
        Exemplo: todo sort  

    - raw [todo/done]  
        imprime apenas as tarefas concluídas/não concluídas em texto puro, útil para scripts  
        Exemplo: todo raw done  
";

pub fn help(){
    println!("{}", TODO_HELP);
}
