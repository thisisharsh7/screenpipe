fn split_compound(text: &str) -> String {
    let result = regex::Regex::new(r"([a-z])([A-Z])").unwrap().replace_all(text, "$1 $2");
    let result = regex::Regex::new(r"([0-9])([a-zA-Z])").unwrap().replace_all(&result, "$1 $2");
    let result = regex::Regex::new(r"([a-zA-Z])([0-9])").unwrap().replace_all(&result, "$1 $2");
    result.into_owned()
}

fn expand_search_query(query: &str) -> String {
    let query = query.trim();
    if query.is_empty() {
        return String::new();
    }

    let expanded_terms: Vec<String> = query
        .split_whitespace()
        .flat_map(|word| {
            let cleaned = word.replace(&['\\', '"'][..], "");
            let split = split_compound(&cleaned);
            let parts: Vec<&str> = split.split_whitespace().collect();

            if parts.len() > 1 {
                let mut terms = vec![format!("\"{}\"*", cleaned)];
                for part in parts {
                    if part.len() >= 2 {
                        terms.push(format!("\"{}\"*", part));
                    }
                }
                terms
            } else {
                vec![format!("\"{}\"*", cleaned)]
            }
        })
        .collect();

    if expanded_terms.len() == 1 {
        expanded_terms[0].clone()
    } else {
        format!("({})", expanded_terms.join(" OR "))
    }
}

fn main() {
    println!("{}", expand_search_query("zoom.us"));
}
