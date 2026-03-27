use sqlx::{sqlite::SqlitePoolOptions, Row};
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await?;

    sqlx::query("CREATE VIRTUAL TABLE frames_fts USING fts5(full_text, app_name, window_name, browser_url)")
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO frames_fts (full_text, app_name) VALUES ('some text', 'zoom.us')")
        .execute(&pool)
        .await?;

    let q = "app_name:\"zoom.us\"";
    println!("Testing q: {}", q);
    let result = sqlx::query("SELECT full_text FROM frames_fts WHERE frames_fts MATCH ?")
        .bind(q)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(rows) => println!("Success, found {} rows", rows.len()),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
