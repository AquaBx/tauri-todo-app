use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

#[derive(Default)]
pub struct TodoState {
    todos: Mutex<Vec<Todo>>,
}

fn get_todos_file_path() -> std::path::PathBuf {
let home = std::env::var("HOME").or(std::env::var("USERPROFILE")).expect("error");
    let mut path = std::path::PathBuf::from(home);
    path.push(".todotauriapp");
    fs::create_dir_all(&path).expect("error");
    path.push("todos.json");
    path
}

#[tauri::command]
fn load_todos(state: State<TodoState>) -> Result<Vec<Todo>, String> {
    let path = get_todos_file_path();
    
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Todo>>(&content) {
                    Ok(loaded_todos) => {
                        let mut todos = state.todos.lock().unwrap();
                        *todos = loaded_todos.clone();
                        Ok(loaded_todos)
                    },
                    Err(e) => Err(format!("error: {}", e))
                }
            },
            Err(e) => Err(format!("error: {}", e))
        }
    } else {
        Ok(Vec::new())
    }
}

#[tauri::command]
fn save_todos(todos: Vec<Todo>, state: State<TodoState>) -> Result<(), String> {
    let path = get_todos_file_path();
    
    {
        let mut state_todos = state.todos.lock().unwrap();
        *state_todos = todos.clone();
    }
    
    match serde_json::to_string_pretty(&todos) {
        Ok(json) => {
            match fs::write(&path, json) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("error: {}", e))
            }
        },
        Err(e) => Err(format!("error: {}", e))
    }
}

#[tauri::command]
fn add_todo(text: String, state: State<TodoState>) -> Result<Todo, String> {
    let mut todos = state.todos.lock().unwrap();
    
    let next_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    
    let new_todo = Todo {
        id: next_id,
        text,
        completed: false,
    };
    
    todos.push(new_todo.clone());
    
    drop(todos);
    match serde_json::to_string_pretty(&state.todos.lock().unwrap().clone()) {
        Ok(json) => {
            match fs::write(get_todos_file_path(), json) {
                Ok(_) => Ok(new_todo),
                Err(e) => Err(format!("error: {}", e))
            }
        },
        Err(e) => Err(format!("error: {}", e))
    }
}

#[tauri::command]
fn toggle_todo(id: u32, state: State<TodoState>) -> Result<(), String> {
    let mut todos = state.todos.lock().unwrap();
    
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        
        drop(todos);
        match serde_json::to_string_pretty(&state.todos.lock().unwrap().clone()) {
            Ok(json) => {
                match fs::write(get_todos_file_path(), json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("error: {}", e))
                }
            },
            Err(e) => Err(format!("error: {}", e))
        }
    } else {
        Err(format!("id {} error", id))
    }
}

#[tauri::command]
fn delete_todo(id: u32, state: State<TodoState>) -> Result<(), String> {
    let mut todos = state.todos.lock().unwrap();
    
    let index = todos.iter().position(|t| t.id == id);
    
    match index {
        Some(idx) => {
            todos.remove(idx);
            
            drop(todos);
            match serde_json::to_string_pretty(&state.todos.lock().unwrap().clone()) {
                Ok(json) => {
                    match fs::write(get_todos_file_path(), json) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("error: {}", e))
                    }
                },
                Err(e) => Err(format!("error: {}", e))
            }
        },
        None => Err(format!("id {} error", id))
    }
}

#[tauri::command]
fn get_todos(state: State<TodoState>) -> Vec<Todo> {
    state.todos.lock().unwrap().clone()
}

fn main() {
    tauri::Builder::default()
        .manage(TodoState::default())
        .invoke_handler(tauri::generate_handler![
            load_todos,
            save_todos,
            add_todo,
            toggle_todo,
            delete_todo,
            get_todos
        ])
        .run(tauri::generate_context!())
        .expect("error");
}