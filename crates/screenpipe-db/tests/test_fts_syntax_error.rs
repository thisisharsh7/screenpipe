use chrono::Utc;
use screenpipe_db::{ContentType, DatabaseManager, OcrEngine};
use std::sync::Arc;

#[tokio::test]
async fn test_fts_syntax_error() {
    let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./src/migrations")
        .run(&db.pool)
        .await
        .unwrap();

    let res = db
        .search(
            "zoom.us",
            ContentType::All,
            10,
            0,
            None,
            None,
            Some("zoom.us"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

    println!("{:?}", res);
    assert!(res.is_ok());
}
