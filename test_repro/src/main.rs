use screenpipe_db::{DatabaseManager, ContentType, Order};

#[tokio::main]
async fn main() {
    let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
    let res = db.search_for_grouping(
        "zoom.us", 
        500, 
        0, 
        None, 
        None, 
        true, 
        Order::Descending, 
        Some(vec!["zoom.us".to_string()]), 
        Some(30)
    ).await;
    if let Err(e) = res { println!("ERROR: {}", e); } else { println!("OK"); }
}
