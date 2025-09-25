use eframe::egui;
use eframe::App;
use eframe::NativeOptions;
use egui::{Vec2, ViewportBuilder};
use crate::libs::*;
use std::sync::{Arc, Mutex};

pub struct TodoApp {
    todo: Arc<Mutex<Todo>>,
    new_task: String,
    edit_task: String,
    edit_index: String,
    select_index: String,
    status_messages: String,
    show_raw_todo: bool,
    show_raw_done: bool,
    //error_message: String,
    dark_mode: bool,
}

impl TodoApp {
    pub fn new(todo: Todo) -> Self {
        Self {
            todo: Arc::new(Mutex::new(todo)),
            new_task: String::new(),
            edit_task: String::new(),
            edit_index: String::new(),
            select_index: String::new(),
            status_messages: String::new(),
            show_raw_done: false,
            show_raw_todo: false,
            //error_message: String::new(),
            dark_mode: true,
        }
    }

    fn set_status(&mut self, message: String){
        self.status_messages = message;
    }

    fn refresh_todo(&mut self){
        if let Ok(mut todo) = self.todo.lock(){
            if let Ok(new_todo) = Todo::new(){
                *todo = new_todo;
            }
        }
    }
}

pub fn run(todo: Todo) {
    let app = TodoApp::new(todo);
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            //.with_always_on_top(false)
            .with_maximized(false)
            .with_decorations(true)
            .with_drag_and_drop(true)
            //.with_icon(None)
            //.with_position(None)
            .with_inner_size(Vec2::new(800.0, 600.0))
            //.with_min_inner_size(None)
            //.with_max_inner_size(None)
            .with_resizable(true)
            .with_transparent(false),
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        ..Default::default()
    };

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
                ui.label("Tema:");
                ui.radio_value(&mut self.dark_mode, true, "Escuro");
                ui.radio_value(&mut self.dark_mode, false, "Claro");
            });

            if self.dark_mode{
                ctx.set_visuals(egui::Visuals::dark());
            } else {
                ctx.set_visuals(egui::Visuals::light());
            }

            if !self.status_messages.is_empty() {
                ui.colored_label(egui::Color32::from_rgb(0, 150, 0), &self.status_messages);
                ui.separator();
            }
            let mut add_clicked = false;
            let mut task_to_add = String::new();
            
            ui.horizontal(|ui| {
                ui.label("Nova tarefa:");
                ui.add(egui::TextEdit::singleline(&mut self.new_task).desired_width(150.0));
                //ui.text_edit_singleline(&mut self.new_task)
                if ui.button("Adicionar").clicked() && !self.new_task.is_empty() {
                    add_clicked = true;
                    task_to_add = self.new_task.clone();
                }
            });

            if add_clicked {
                if let Ok(todo) = self.todo.lock() {
                    todo.add(&[task_to_add.clone()]);
                }

                self.set_status(format!("Tarefa '{}' adicionada!", task_to_add));
                self.new_task.clear();
                self.refresh_todo();
            }

            ui.separator();

            // Edit task section
            let mut edit_clicked = false;
            let mut edit_data = (String::new(), String::new());

            ui.horizontal(|ui|{
                ui.label("Editar Tarefa - Índice:");
                ui.add(egui::TextEdit::singleline(&mut self.edit_index).desired_width(150.0));
                //ui.text_edit_singleline(&mut self.edit_index);
                ui.label("Nova tarefa:");
                ui.add(egui::TextEdit::singleline(&mut self.edit_task).desired_width(150.0));
                //ui.text_edit_singleline(&mut self.edit_task);
                if ui.button("Editar").clicked() && !self.edit_index.is_empty() && !self.edit_task.is_empty() {
                    edit_clicked = true;
                    edit_data = (self.edit_index.clone(), self.edit_task.clone());
                }
            });

            if edit_clicked {
                if let Ok(todo) = self.todo.lock() {
                    todo.edit(&[edit_data.0.clone(), edit_data.1.clone()]);
                }
                    self.set_status(format!("Tarefa {} editada para '{}'!", edit_data.0, edit_data.1));
                    self.edit_index.clear();
                    self.edit_task.clear();
                    self.refresh_todo();
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Índices (separados por espaço):");
                ui.add(egui::TextEdit::singleline(&mut self.select_index).desired_width(150.0));
                //ui.text_edit_singleline(&mut self.select_index);
            });

            let mut done_clicked = false;
            let mut remove_clicked = false;
            let mut indices_data = String::new();

            ui.horizontal(|ui| {
                if ui.button("Marcar/Desmarcar").clicked() && !self.select_index.is_empty() {
                    done_clicked = true;
                    indices_data = self.select_index.clone();
                }

                if ui.button("Remover").clicked() && !self.select_index.is_empty() {
                    remove_clicked = true;
                    indices_data = self.select_index.clone();
                }
            });

            if done_clicked {
                let indices: Vec<String> = indices_data.split_whitespace().map(|s| s.to_string()).collect();
                if let Ok(todo) = self.todo.lock() {
                    todo.done(&indices);
                }
                    self.set_status("Status das tarefas alterado!".to_string());
                    self.refresh_todo();
            }

            if remove_clicked {
                let indices: Vec<String> = indices_data.split_whitespace().map(|s| s.to_string()).collect();
                if let Ok(todo) = self.todo.lock() {
                    todo.remove(&indices);
                }
                    self.set_status("Tarefas removidas!".to_string());
                    self.select_index.clear();
                    self.refresh_todo();
            }

            ui.separator();

            let mut sort_clicked = false;
            let mut refresh_clicked = false;
            let mut reset_clicked = false;
            let mut restore_clicked = false;

            ui.horizontal(|ui| {
                if ui.button("Ordenar").clicked() {
                    sort_clicked = true;
                }

                if ui.button("Atualizar Lista").clicked() {
                    refresh_clicked = true;
                }

                if ui.button("Reset").clicked() {
                    reset_clicked = true;
                }

                if ui.button("Restaurar").clicked() {
                    restore_clicked = true;
                }
            });

            if sort_clicked {
                if let Ok(todo) = self.todo.lock() {
                    todo.sort();
                }
                    self.set_status("Lista ordenada (pendentes primeiro)!".to_string());
                    self.refresh_todo();
            }

            if refresh_clicked {
                self.refresh_todo();
                self.set_status("Lista atualizada!".to_string());
            }

            if reset_clicked {
                if let Ok(todo) = self.todo.lock() {
                    todo.reset();
                }
                    self.set_status("Lista resetada (backup criado)!".to_string());
                    self.refresh_todo();
            }

            if restore_clicked {
                if let Ok(todo) = self.todo.lock() {
                    todo.restore();
                }
                    self.set_status("Lista restaurada do backup!".to_string());
                    self.refresh_todo();
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Ver TODOs Pendentes").clicked() {
                    self.show_raw_todo = !self.show_raw_todo;
                    self.show_raw_done = false;
                }
                if ui.button("Ver TODOs Completos").clicked() {
                    self.show_raw_done = !self.show_raw_done;
                    self.show_raw_todo = false;
                }
            });

            ui.separator();

            if self.show_raw_todo {
                ui.label("📋 Tarefas Pendentes:");
                if let Ok(todo) = self.todo.lock() {
                    egui::ScrollArea::vertical()
                        .id_salt("pending_scroll")
                        .show(ui, |ui| {
                        for line in &todo.todo {
                            let entry = Entry::read_line(line);
                            if !entry.done {
                                ui.label(&entry.todo_entry);
                            }
                        }
                    });
                }
                ui.separator();
            }

            if self.show_raw_done {
                ui.label("Tarefas Completas:");
                if let Ok(todo) = self.todo.lock() {
                    egui::ScrollArea::vertical()
                        .id_salt("done_scroll")
                        .show(ui, |ui| {
                        for line in &todo.todo {
                            let entry = Entry::read_line(line);
                            if entry.done {
                                ui.label(&entry.todo_entry);
                            }
                        }
                    });
                }
                ui.separator();
            }

            ui.label("Lista de Tarefas:");
            
            egui::ScrollArea::vertical()
                .id_salt("main_scroll")
                .show(ui, |ui| {
                    if let Ok(todo) = self.todo.lock() {
                        for (index, line) in todo.todo.iter().enumerate() {
                            let entry = Entry::read_line(line);
                            let status_icon = if entry.done { "✅" } else { "⭕" };
                            let task_text = if entry.done {
                                format!("{}", entry.todo_entry)
                            } else {
                                entry.todo_entry.clone()
                            };
                            
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", index + 1));
                                ui.label(status_icon);
                                
                                // Make completed tasks appear dimmed
                                if entry.done {
                                    ui.colored_label(egui::Color32::GRAY, task_text);
                                } else {
                                    ui.label(task_text);
                                }
                            });
                        }
                    }
            });
        });
    }
}
