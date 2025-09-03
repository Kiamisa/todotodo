use eframe::egui;
use eframe::App;
use crate::libs::*;
use std::sync::{Arc, Mutex};

pub struct TodoApp {
    todo: Arc<Mutex<Todo>>,
    new_task: String,
}

impl TodoApp {
    pub fn new(todo: Todo) -> Self {
        Self {
            todo: Arc::new(Mutex::new(todo)),
            new_task: String::new(),
        }
    }
}

pub fn run(todo: Todo) {
    let app = TodoApp::new(todo);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "TodoTodo GUI",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .expect("Erro ao iniciar a GUI");
}

impl App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TodoTodo GUI");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Adicionar").clicked() && !self.new_task.is_empty() {
                    if let Ok(todo) = self.todo.lock() {
                        todo.add(&[self.new_task.clone()]);
                    }
                    self.new_task.clear();
                }
            });

            ui.separator();

            // Lista de tarefas
            if let Ok(todo) = self.todo.lock() {
                for (index, line) in todo.todo.iter().enumerate() {
                    let entry = Entry::read_line(line);
                    let status = if entry.done { "[*]" } else { "[ ]" };
                    ui.label(format!("{} {} {}", index + 1, status, entry.todo_entry));
                }
            }
        });
    }
}

