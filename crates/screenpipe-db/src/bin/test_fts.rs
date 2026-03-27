use sqlx::sqlite::SqlitePoolOptions;
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    sqlx::query("CREATE VIRTUAL TABLE frames_fts USING fts5(app_name, window_name, text)")
        .execute(&pool)
        .await?;

    let query_str = "app_name:\"zoom.us\"";

    let res = sqlx::query("SELECT * FROM frames_fts WHERE frames_fts MATCH ?")
        .bind(query_str)
        .fetch_all(&pool)
        .await;

    match res {
        Ok(_) => println!("Res 1 OK"),
        Err(e) => println!("Res 1 Error: {}", e),
    }

    let query_str2 = "app_name:zoom.us";

    let res2 = sqlx::query("SELECT * FROM frames_fts WHERE frames_fts MATCH ?")
        .bind(query_str2)
        .fetch_all(&pool)
        .await;

    match res2 {
        Ok(_) => println!("Res 2 OK"),
        Err(e) => println!("Res 2 Error: {}", e),
    }

    Ok(())
}
