use chrono::{Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tauri::{Manager, State};

pub(crate) mod database;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: i64,
    uuid: String,
    text: String,
    completed: bool,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    created_at: String,
}
type Todos = Vec<Todo>;

#[tauri::command]
async fn get_todos(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<Todos, String> {
    let todos = database::get_todos(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(todos)
}

#[tauri::command]
async fn add_todo(sqlite_pool: State<'_, sqlx::SqlitePool>, text: &str) -> Result<String, String> {
    let todo = Todo {
        id: 0,
        uuid: Uuid::new_v4().to_string(),
        text: text.to_string(),
        completed: false,
        created_at: Utc::now().to_rfc3339(),
    };
    database::add_todo(&*sqlite_pool, todo)
        .await
        .map_err(|e| e.to_string())?;
    Ok("Todo added".to_string())
}

#[tauri::command]
async fn delete_todo(sqlite_pool: State<'_, sqlx::SqlitePool>, id: i64) -> Result<String, String> {
    database::delete_todo(&*sqlite_pool, id)
        .await
        .map_err(|e| e.to_string())?;
    Ok("Todo deleted".to_string())
}

#[tauri::command]
async fn update_todo(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    id: i64,
    text: &str,
    completed: bool,
) -> Result<String, String> {
    database::update_todo(&*sqlite_pool, id, text, completed)
        .await
        .map_err(|e| e.to_string())?;
    Ok("Todo updated".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // このmain関数はasync fnではないので、asyncな関数を呼ぶのにblock_on関数を使う
    use tauri::async_runtime::block_on;

    // データベースのファイルパス等を設定する
    const DATABASE_DIR: &str = ".simple-todo-db";
    const DATABASE_FILE: &str = "db.sqlite";
    // ユーザのホームディレクトリ直下にデータベースのディレクトリを作成する
    // もし、各OSで標準的に使用されるアプリ専用のデータディレクトリに保存したいなら
    // directoriesクレートの`ProjectDirs::data_dir`メソッドなどを使うとよい
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir
    let home_dir = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        // ホームディレクトリが取得できないときはカレントディレクトリを使う
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let database_dir = home_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    // データベースファイルが存在するかチェックする
    let db_exists = std::fs::metadata(&database_file).is_ok();
    // 存在しないなら、ファイルを格納するためのディレクトリを作成する
    if !db_exists {
        std::fs::create_dir(&database_dir)?;
    }
    let database_dir_str = dunce::canonicalize(&database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!("sqlite://{}/{}", database_dir_str, DATABASE_FILE);

    // SQLiteのコネクションプールを作成する
    let sqlite_pool = block_on(database::create_sqlite_pool(&database_url))?;

    //  データベースファイルが存在しなかったなら、マイグレーションSQLを実行する
    if !db_exists {
        block_on(database::migrate_database(&sqlite_pool))?;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            delete_todo,
            update_todo,
        ])
        // ハンドラからコネクションプールにアクセスできるよう、登録する
        .setup(|app| {
            app.manage(sqlite_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
