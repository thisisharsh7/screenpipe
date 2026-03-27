use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE VIRTUAL TABLE frames_fts USING fts5(
            full_text, app_name, window_name, browser_url
        )",
        (),
    )?;
    
    conn.execute(
        "INSERT INTO frames_fts (full_text, app_name) VALUES ('some text', 'zoom.us')",
        (),
    )?;

    // test app_name:"zoom.us"
    let q1 = "app_name:\"zoom.us\"";
    let mut stmt = conn.prepare("SELECT * FROM frames_fts WHERE frames_fts MATCH ?1")?;
    let mut rows = stmt.query([q1])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        println!("Got: {}", name);
    }

    Ok(())
}
