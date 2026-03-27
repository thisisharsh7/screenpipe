fn sanitize_fts5_query(query: &str) -> String {
    query
        .split_whitespace()
        .filter_map(|token| {
            let cleaned = token.replace(['\\', '"'], "");
            if cleaned.is_empty() {
                return None;
            }
            Some(format!("\"{}\"", cleaned))
        })
        .collect::<Vec<_>>()
        .join(" ")
}
fn value_to_fts5_column_query(column: &str, value: &str) -> String {
    sanitize_fts5_query(value)
        .split_whitespace()
        .map(|token| format!("{}:{}", column, token))
        .collect::<Vec<_>>()
        .join(" ")
}
fn main() {
    println!("{}", value_to_fts5_column_query("app_name", "zoom.us"));
}
