use screenpipe_db::DatabaseManager;
use crate::snapshot_compaction::run_compaction_cycle;

#[tokio::test]
async fn test_run_compaction_cycle_missing_column() {
    let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
    // Drop snapshot_path column to simulate missing migration
    sqlx::query("ALTER TABLE frames DROP COLUMN snapshot_path")
        .execute(&db.pool)
        .await
        .unwrap();

    let result = run_compaction_cycle(&db, "high", 100, &None).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
