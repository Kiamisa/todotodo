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

    

}
