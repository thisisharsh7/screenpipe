use sqlx::sqlite::SqlitePoolOptions;
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    sqlx::query("CREATE VIRTUAL TABLE frames_fts USING fts5(app_name, window_name, text)")
        .execute(&pool)
        .await?;

    let query_str = "\"zoom.us\" app_name:\"zoom.us\"";

    let res = sqlx::query("SELECT * FROM frames_fts WHERE frames_fts MATCH ?")
        .bind(query_str)
        .fetch_all(&pool)
        .await;

    match res {
        Ok(_) => println!("Res OK"),
        Err(e) => println!("Res Error: {}", e),
    }

    Ok(())
}
