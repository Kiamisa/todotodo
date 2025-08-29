use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::{env, process};

pub struct Entry {
    pub todo_entry: String,
    pub done: bool,
}

impl Entry {
    pub fn new(todo_entry: String, done: bool) -> Self {
        Self { todo_entry, done }
    }

    pub fn file_line(&self) -> String {
        let symbol = if self.done { "[*] " } else { "[ ] " };
        format!("{}{}\n", symbol, self.todo_entry)
    }

    pub fn list_line(&self, number: usize) -> String {
        let todo_entry = if self.done {
            self.todo_entry.strikethrough().to_string()
        } else {
            self.todo_entry.clone()
        };
        format!("{number} {todo_entry}\n")
    }

    pub fn read_line(line: &String) -> Self {
        let done = line.len() >= 4 && &line[..4] == "[*] ";
        let todo_entry = if line.len() > 4 {
            line[4..].to_string()
        } else {
            line.clone()
        };
        Self { todo_entry, done }
    }

    pub fn raw_line(&self) -> String {
        format!("{}\n", self.todo_entry)
    }
}

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: PathBuf,
    pub todo_bak: PathBuf,
    pub no_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        // Determina o diretório home cross-platform
        let home_dir = dirs::home_dir().ok_or("Não foi possível determinar o diretório home")?;

        // Caminho do TODO principal - with better error handling
        let todo_path = match env::var("TODO_PATH") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                let legacy = home_dir.join("TODO");
                if legacy.exists() {
                    legacy
                } else {
                    home_dir.join(".todo")
                }
            }
        };

        // Caminho do backup - with better error handling
        let todo_bak = match env::var("TODO_BAK_DIR") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                if cfg!(windows) {
                    home_dir.join("todo.bak")
                } else {
                    PathBuf::from("/tmp/todo.bak")
                }
            }
        };

        // Check if backup is disabled
        let no_backup = match env::var("TODO_NOBACKUP") {
            Ok(_) => true,
            Err(_) => false,
        };

        // Cria o arquivo se não existir
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .map_err(|e| format!("Não foi possível abrir o arquivo TODO: {}", e))?;

        let mut buf_reader = BufReader::new(&todofile);
        let mut contents = String::new();
        
        // Replace unwrap() with proper error handling
        buf_reader.read_to_string(&mut contents)
            .map_err(|e| format!("Erro ao ler o arquivo TODO: {}", e))?;

        let todo = contents.lines().map(str::to_string).collect();

        Ok(Self {
            todo,
            todo_path,
            todo_bak,
            no_backup,
        })
    }

    pub fn list(&self) {
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();

        for (number, task) in self.todo.iter().enumerate() {
            let entry = Entry::read_line(task);
            let line = entry.list_line(number + 1);
            data.push_str(&line);
        }

        if let Err(e) = writer.write_all(data.as_bytes()) {
            eprintln!("Falha na gravação: {}", e);
        }
    }

    pub fn raw(&self, arg: &[String]) {
        if arg.len() != 1 {
            eprintln!("Uso: todo raw [todo/done]");
            return;
        }
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout);

        let arg = &arg[0];

        for task in self.todo.iter() {
            let entry = Entry::read_line(task);
            if (entry.done && arg == "done") || (!entry.done && arg == "todo") {
                if let Err(e) = writer.write_all(entry.raw_line().as_bytes()) {
                    eprintln!("Falha na gravação: {}", e);
                    break;
                }
            }
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("Precisa colocar alguma tarefa");
            process::exit(1);
        }

        let todofile = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Não foi possível abrir o arquivo: {}", e);
                process::exit(1);
            }
        };

        let mut buffer = BufWriter::new(todofile);

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }
            let entry = Entry::new(arg.to_string(), false);
            if let Err(e) = buffer.write_all(entry.file_line().as_bytes()) {
                eprintln!("Falha na gravação: {}", e);
                process::exit(1);
            }
        }
    }

    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("Precisa remover alguma tarefa");
            process::exit(1);
        }

        let todofile = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Não foi possível abrir o arquivo: {}", e);
                process::exit(1);
            }
        };

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }
            if let Err(e) = buffer.write_all(format!("{}\n", line).as_bytes()) {
                eprintln!("Falha na gravação: {}", e);
                process::exit(1);
            }
        }
    }

    fn remove_file(&self) {
        if let Err(e) = fs::remove_file(&self.todo_path) {
            eprintln!("Erro ao apagar arquivo: {}", e)
        }
    }

    pub fn reset(&self) {
        if !self.no_backup {
            if fs::copy(&self.todo_path, &self.todo_bak).is_ok() {
                self.remove_file()
            } else {
                eprintln!("Não foi possível fazer backup do arquivo");
            }
        } else {
            self.remove_file();
        }
    }

    pub fn restore(&self) {
        if let Err(e) = fs::copy(&self.todo_bak, &self.todo_path) {
            eprintln!("Não foi possível restaurar o arquivo de backup: {}", e);
            process::exit(1);
        }
    }

    pub fn sort(&self) {
        let mut todo = String::new();
        let mut done = String::new();

        for line in self.todo.iter() {
            let entry = Entry::read_line(line);
            if entry.done {
                done.push_str(&format!("{}\n", line));
            } else {
                todo.push_str(&format!("{}\n", line));
            }
        }

        let newtodo = format!("{}{}", todo, done);

        let mut todofile = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Não foi possível abrir o arquivo: {}", e);
                process::exit(1);
            }
        };

        if let Err(e) = todofile.write_all(newtodo.as_bytes()) {
            eprintln!("Falha ao salvar o arquivo: {}", e);
            process::exit(1);
        }
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("Precisa de pelo menos 1 argumento");
            process::exit(1);
        }

        let todofile = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Não foi possível abrir o arquivo: {}", e);
                process::exit(1);
            }
        };

        let mut buffer = BufWriter::new(todofile);
        let mut data = String::new();

        for (pos, line) in self.todo.iter().enumerate() {
            let mut entry = Entry::read_line(line);
            let line = if args.contains(&(pos + 1).to_string()) {
                entry.done = !entry.done;
                entry.file_line()
            } else {
                format!("{}\n", line)
            };
            data.push_str(&line);
        }
        
        if let Err(e) = buffer.write_all(data.as_bytes()) {
            eprintln!("Falha na gravação: {}", e);
            process::exit(1);
        }
    }

    pub fn edit(&self, args: &[String]) {
        if args.len() != 2 {
            eprintln!("A edição precisa de exatamente 2 argumentos");
            process::exit(1);
        }

        let todofile = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Não foi possível abrir o arquivo: {}", e);
                process::exit(1);
            }
        };

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            let line = if args[0] == (pos + 1).to_string() {
                let mut entry = Entry::read_line(line);
                entry.todo_entry = args[1].clone();
                entry.file_line()
            } else {
                format!("{}\n", line)
            };
            if let Err(e) = buffer.write_all(line.as_bytes()) {
                eprintln!("Falha na gravação: {}", e);
                process::exit(1);
            }
        }
    }
}

const TODO_HELP: &str = "Uso: todo [COMANDO] [ARGUMENTOS]
Todo é um organizador de tarefas super rápido e simples escrito em Rust

Exemplo: todo list

Comandos disponíveis:
    - mk [TAREFA/S]
    - edit [ÍNDICE] [TAREFA/S EDITADA/S]
    - list
    - done [ÍNDICE]
    - rm [ÍNDICE]
    - reset
    - restore
    - sort
    - raw [todo/done]
";

pub fn help() {
    println!("{}", TODO_HELP);
}
