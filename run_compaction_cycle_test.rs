    #[tokio::test]
    async fn test_run_compaction_cycle_missing_column() {
        use screenpipe_db::DatabaseManager;
        let db = DatabaseManager::new("sqlite::memory:").await.unwrap();
        // Create frames table WITHOUT snapshot_path
        sqlx::query(
            r#"
            CREATE TABLE frames (
                id INTEGER PRIMARY KEY,
                device_name TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )
            "#,
        )
        .execute(&db.pool)
        .await
        .unwrap();

        let result = run_compaction_cycle(&db, "720p", 100, &None).await;
        // It should return Ok(0) and NOT error
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
