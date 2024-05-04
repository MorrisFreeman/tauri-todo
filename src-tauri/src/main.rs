use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: String,
    text: String,
    completed: bool,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    created_at: DateTime<Utc>,
}
type Todos = Vec<Todo>;
static mut TODOS: Todos = Vec::new();

#[tauri::command]
fn post_todo(text: &str) -> String {
    println!("Adding todo: {}", text);
    unsafe {
        TODOS.push(Todo {
            id: Uuid::new_v4().to_string(),
            text: text.to_string(),
            completed: false,
            created_at: Utc::now(),
        });
    }
    "Todo added".to_string()
}

#[tauri::command]
fn get_todos() -> Todos {
    println!("Getting todos");
    unsafe {
        TODOS.clone()
    }
}

#[tauri::command]
fn delete_todo(id: &str) -> String {
    println!("Deleting todo: {}", id);
    unsafe {
        TODOS.retain(|todo| todo.id != id);
    }
    "Todo deleted".to_string()
}

#[tauri::command]
fn update_todo(id: &str, text: &str, completed: bool) -> String {
    println!("Updating todo: {}", id);
    unsafe {
        for todo in TODOS.iter_mut() {
            if todo.id == id {
                todo.text = text.to_string();
                todo.completed = completed;
            }
        }
    }
    "Todo updated".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![post_todo, get_todos, delete_todo, update_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
