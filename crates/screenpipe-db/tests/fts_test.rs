use screenpipe_db::{ContentType, DatabaseManager};
use std::sync::Arc;

#[tokio::test]
async fn test_fts_dot() -> Result<(), anyhow::Error> {
    let db = DatabaseManager::new("sqlite::memory:").await?;
    let res = db
        .search(
            "",
            ContentType::All,
            10,
            0,
            None,
            None,
            Some("zoom.us"), // app_name
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

    match res {
        Ok(r) => println!("Ok, {} results", r.len()),
        Err(e) => {
            println!("Error: {:?}", e);
            panic!("Search failed");
        }
    }
    Ok(())
}
