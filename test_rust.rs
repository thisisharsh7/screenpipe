fn main() {
    let value = "zoom.us";
    let sanitized = sanitize_fts5_query(value);
    println!("Sanitized: {}", sanitized);
    let mapped: Vec<_> = sanitized.split_whitespace().map(|t| format!("app_name:{}", t)).collect();
    println!("Mapped: {}", mapped.join(" "));
}

pub fn sanitize_fts5_query(query: &str) -> String {
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
