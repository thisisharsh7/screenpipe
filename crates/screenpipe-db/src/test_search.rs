use crate::db::DatabaseManager;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
    let res = db.search(
        "", 
        crate::db::ContentType::All, 
        10, 0, 
        None, None, 
        Some("zoom.us"), None, 
        None, None, 
        None, None, 
        None, None,
        None, None, None
    ).await;
    match res {
        Ok(_) => println!("OK"),
        Err(e) => println!("ERROR: {}", e),
    }
}
