#[cfg(test)]
mod test_compaction {
    use super::*;
    use screenpipe_db::DatabaseManager;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_run_compaction_cycle_missing_column() {
        let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
        // Create frames table WITHOUT snapshot_path
        sqlx::query(
            "CREATE TABLE frames (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device_name TEXT NOT NULL,
                timestamp DATETIME NOT NULL
            )",
        )
        .execute(&db.pool)
        .await
        .unwrap();

        let result = run_compaction_cycle(&db, "high", 100, &None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
}
