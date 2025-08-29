# Todo-Todo

**Todo-Todo** é um organizador de tarefas simples e rápido escrito em Rust. Ele permite gerenciar suas tarefas diretamente pelo terminal de forma eficiente, com funcionalidades para adicionar, remover, marcar como concluídas e ordenar suas tarefas.

> 🚧 A interface gráfica está em desenvolvimento.

---

## Funcionalidades

- **Adicionar tarefas** (`add` / `mk`)
- **Editar tarefas** (`edit`)
- **Listar tarefas** (`list`)
- **Marcar como concluídas** (`done`)
- **Remover tarefas** (`rm`)
- **Ordenar tarefas concluídas e não concluídas** (`sort`)
- **Resetar ou restaurar o arquivo de tarefas** (`reset` / `restore`)
- **Mostrar apenas tarefas concluídas ou pendentes** (`raw`)
- **Ajuda** (`help`, `--help`, `-h`)

---

## Como usar

1. Compile o projeto:

```bash
cargo build
````

2. Execute o binário diretamente pelo terminal:

```bash
# Adicionar uma tarefa
.\target\debug\todo-todo.exe mk "Comprar leite"

# Listar tarefas
.\target\debug\todo-todo.exe list

# Marcar tarefa como concluída
.\target\debug\todo-todo.exe done 1

# Remover tarefa
.\target\debug\todo-todo.exe rm 1

# Ordenar tarefas
.\target\debug\todo-todo.exe sort

# Mostrar apenas tarefas pendentes
.\target\debug\todo-todo.exe raw todo

# Mostrar apenas tarefas concluídas
.\target\debug\todo-todo.exe raw done
```

> No Linux ou MacOS, o binário será `./target/debug/todo-todo`.

---

## Estrutura do Projeto

* **main.rs**: Ponto de entrada da aplicação. Faz o parsing dos comandos e argumentos.
* **lib.rs**: Contém toda a lógica de gerenciamento das tarefas (`Todo` e `Entry`).
* **target/**: Pasta onde o Cargo gera o binário e arquivos intermediários.

---

## Próximos Passos

* Desenvolvimento da **interface gráfica** para tornar o gerenciamento de tarefas ainda mais amigável.

---

