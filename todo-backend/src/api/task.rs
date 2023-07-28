use crate::models::task::{NewTask, Task};
use axum::{
    extract::{Path, State},
    Form, Json,
};
use axum_error::Result;
use sqlx::SqlitePool;

pub async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Task>>> {
    let tasks = sqlx::query_as!(Task, "SELECT id, description, done FROM tasks ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(tasks))
}

pub async fn create(State(pool): State<SqlitePool>, Form(task): Form<NewTask>) -> Result<()> {
    sqlx::query!(
        "INSERT INTO tasks (description) VALUES (?)",
        task.description,
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn read(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Task>> {
    // Read todo
    let task = sqlx::query_as!(
        Task,
        "SELECT id, description, done FROM tasks WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(task))
}

pub async fn update(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Form(task): Form<Task>,
) -> Result<()> {
    // Update todo
    sqlx::query!(
        "UPDATE tasks SET description = ?, DONE = ? WHERE id = ?",
        task.description,
        task.done,
        id
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<()> {
    // Update todo
    sqlx::query!("DELETE FROM tasks WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(())
}
