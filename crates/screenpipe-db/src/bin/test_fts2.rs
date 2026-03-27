use screenpipe_db::text_normalizer::value_to_fts5_column_query;

fn main() {
    println!("{}", value_to_fts5_column_query("app_name", "zoom.us"));
}
