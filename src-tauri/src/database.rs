use std::{collections::BTreeMap, str::FromStr};
use futures::TryStreamExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Row, SqlitePool,
};
use crate::{Todo};

/// このモジュール内の関数の戻り値型
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// SQLiteのコネクションプールを作成して返す
pub(crate) async fn create_sqlite_pool(database_url: &str) -> DbResult<SqlitePool> {
    // コネクションの設定
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        // DBが存在しないなら作成する
        .create_if_missing(true)
        // トランザクション使用時の性能向上のため、WALを使用する
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    // 上の設定を使ってコネクションプールを作成する
    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

/// マイグレーションを行う
pub(crate) async fn migrate_database(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./db").run(pool).await?;
    Ok(())
}

pub(crate) async fn get_todos(pool: &SqlitePool) -> DbResult<Vec<Todo>> {
    const SQL: &str = "SELECT * FROM todos ORDER BY id ASC";
    let mut rows = sqlx::query(SQL).fetch(pool);

    let mut todos = BTreeMap::new();
    while let Some(row) = rows.try_next().await? {
        let id: i64 = row.try_get("id")?;
        let uuid: &str = row.try_get("uuid")?;
        let text: &str = row.try_get("text")?;
        let completed: bool = row.try_get("completed")?;
        let created_at: &str = row.try_get("created_at")?;
        todos.insert(
            id,
            Todo {
                id,
                uuid: uuid.to_string(),
                text: text.to_string(),
                completed,
                created_at: created_at.to_string(),
            },
        );
    }

    Ok(todos.into_iter().map(|(_k, v)| v).collect())
}

pub(crate) async fn add_todo(pool: &SqlitePool, todo: Todo) -> DbResult<()> {
    const SQL: &str = "INSERT INTO todos (uuid, text, completed, created_at) VALUES (?, ?, ?, ?)";
    sqlx::query(SQL)
        .bind(todo.uuid)
        .bind(todo.text)
        .bind(todo.completed)
        .bind(todo.created_at)
        .execute(pool)
        .await?;

    Ok(())
}

pub(crate) async fn delete_todo(pool: &SqlitePool, id: i64) -> DbResult<()> {
    const SQL: &str = "DELETE FROM todos WHERE id = ?";
    sqlx::query(SQL).bind(id).execute(pool).await?;
    Ok(())
}

pub(crate) async fn update_todo(
    pool: &SqlitePool,
    id: i64,
    text: &str,
    completed: bool,
) -> DbResult<()> {
    const SQL: &str = "UPDATE todos SET text = ?, completed = ? WHERE id = ?";
    sqlx::query(SQL).bind(text).bind(completed).bind(id).execute(pool).await?;
    Ok(())
}
