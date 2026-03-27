#[tokio::test]
async fn test_search_dot() {
    let db = crate::db::DatabaseManager::new("sqlite::memory:").await.unwrap();
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
        Err(e) => panic!("ERROR: {}", e),
    }
}
