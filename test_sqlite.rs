use sqlx::{sqlite::SqlitePoolOptions, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    sqlx::query("CREATE VIRTUAL TABLE t USING fts5(app_name, content);")
        .execute(&pool)
        .await?;

    let q1 = r#"app_name:"zoom.us""#;
    println!("query: {}", q1);
    match sqlx::query("SELECT * FROM t WHERE t MATCH ?").bind(q1).fetch_all(&pool).await {
        Ok(_) => println!("q1 ok"),
        Err(e) => println!("q1 error: {}", e),
    }

    Ok(())
}
