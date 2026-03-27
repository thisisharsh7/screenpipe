use crate::text_normalizer::{sanitize_fts5_query, value_to_fts5_column_query};

#[test]
fn test_my_fts_query() {
    let q = value_to_fts5_column_query("app_name", "zoom.us");
    println!("QUERY IS: '{}'", q);
    assert_eq!(q, "app_name:\"zoom.us\"");
}
