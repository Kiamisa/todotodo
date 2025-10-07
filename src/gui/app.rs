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
            // --- Seção do Tema (sem alterações) ---
            ui.heading("TodoTodo GUI");
            ui.horizontal(|ui| {
                ui.label("Tema:");
                ui.radio_value(&mut self.dark_mode, true, "Escuro");
                ui.radio_value(&mut self.dark_mode, false, "Claro");
            });

            if self.dark_mode {
                ctx.set_visuals(egui::Visuals::dark());
            } else {
                ctx.set_visuals(egui::Visuals::light());
            }

            // --- Mensagem de Status (sem alterações) ---
            if !self.status_messages.is_empty() {
                let message = self.status_messages.clone(); // Clona para usar no label
                ui.colored_label(egui::Color32::from_rgb(0, 150, 0), message);
            }
            ui.separator();

            // --- NOVO: Início do Formulário com Grid ---
            egui::Grid::new("todo_form")
                .num_columns(3) // Coluna 1: Label, Coluna 2: Input, Coluna 3: Botão(ões)
                .spacing([10.0, 8.0]) // Espaçamento [horizontal, vertical]
                .show(ui, |ui| {
                    // --- Linha 1: Adicionar Tarefa ---
                    ui.label("Nova tarefa:");
                    ui.text_edit_singleline(&mut self.new_task);
                    if ui.button("Adicionar").clicked() {
                        if !self.new_task.is_empty() {
                            if let Ok(todo) = self.todo.lock() {
                                todo.add(&[self.new_task.clone()]);
                            }
                            self.set_status(format!("Tarefa '{}' adicionada!", self.new_task));
                            self.new_task.clear();
                            self.refresh_todo();
                        }
                    }
                    ui.end_row();

                    // --- Linha 2: Editar Tarefa ---
                    ui.label("Editar Tarefa - Índice:");
                    // Célula com múltiplos widgets
                    ui.horizontal(|ui| {
                        // O campo de índice pode ser menor, então vamos dar um tamanho fixo
                        ui.add(egui::TextEdit::singleline(&mut self.edit_index).desired_width(50.0));
                        ui.label("Nova tarefa:");
                        // O campo da nova tarefa deve expandir
                        ui.add(egui::TextEdit::singleline(&mut self.edit_task).desired_width(100.0));
                    });
                    if ui.button("Editar").clicked() {
                        if !self.edit_index.is_empty() && !self.edit_task.is_empty() {
                            if let Ok(todo) = self.todo.lock() {
                                todo.edit(&[self.edit_index.clone(), self.edit_task.clone()]);
                            }
                            self.set_status(format!("Tarefa {} editada para '{}'!", self.edit_index, self.edit_task));
                            self.edit_index.clear();
                            self.edit_task.clear();
                            self.refresh_todo();
                        }
                    }
                    ui.end_row();

                    // --- Linha 3: Marcar/Remover Tarefas ---
                    ui.label("Índices (separados por espaço):");
                    ui.text_edit_singleline(&mut self.select_index);
                    // Célula com múltiplos botões
                    ui.horizontal(|ui| {
                        if ui.button("Marcar/Desmarcar").clicked() {
                            if !self.select_index.is_empty() {
                                let indices: Vec<String> = self.select_index.split_whitespace().map(String::from).collect();
                                if let Ok(todo) = self.todo.lock() {
                                    todo.done(&indices);
                                }
                                self.set_status("Status das tarefas alterado!".to_string());
                                self.select_index.clear(); // Limpa após o uso
                                self.refresh_todo();
                            }
                        }
                        if ui.button("Remover").clicked() {
                            if !self.select_index.is_empty() {
                                let indices: Vec<String> = self.select_index.split_whitespace().map(String::from).collect();
                                if let Ok(todo) = self.todo.lock() {
                                    todo.remove(&indices);
                                }
                                self.set_status("Tarefas removidas!".to_string());
                                self.select_index.clear(); // Limpa após o uso
                                self.refresh_todo();
                            }
                        }
                    });
                    ui.end_row();
                }); // --- Fim do Grid ---

            ui.separator();

            // --- Botões de Ação (agora usam a lógica imediata também) ---
            ui.horizontal(|ui| {
                if ui.button("Ordenar").clicked() {
                    if let Ok(todo) = self.todo.lock() {
                        todo.sort();
                    }
                    self.set_status("Lista ordenada (pendentes primeiro)!".to_string());
                    self.refresh_todo();
                }

                if ui.button("Atualizar Lista").clicked() {
                    self.refresh_todo();
                    self.set_status("Lista atualizada!".to_string());
                }

                if ui.button("Reset").clicked() {
                    if let Ok(todo) = self.todo.lock() {
                        todo.reset();
                    }
                    self.set_status("Lista resetada (backup criado)!".to_string());
                    self.refresh_todo();
                }

                if ui.button("Restaurar").clicked() {
                    if let Ok(todo) = self.todo.lock() {
                        todo.restore();
                    }
                    self.set_status("Lista restaurada do backup!".to_string());
                    self.refresh_todo();
                }
            });

            ui.separator();

            // --- Botões de Visualização (sem grandes alterações) ---
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
            
            // --- O resto do código para exibir as listas permanece o mesmo ---
            // ... (if self.show_raw_todo, if self.show_raw_done, e a lista principal)
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
                ui.label("✅ Tarefas Completas:");
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
                            let task_text = entry.todo_entry.clone();
                            
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", index + 1));
                                ui.label(status_icon);
                                
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
