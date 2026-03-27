import { Database } from "bun:sqlite";

const db = new Database(":memory:");
db.run("CREATE VIRTUAL TABLE frames_fts USING fts5(name, app_name);");
db.run("INSERT INTO frames_fts VALUES ('frame1', 'zoom.us');");

try {
    const rows = db.query("SELECT * FROM frames_fts WHERE frames_fts MATCH 'app_name:\"zoom.us\"'").all();
    console.log("direct:", rows);
} catch (e) { console.log("direct err:", e.message); }

try {
    const rows = db.query("SELECT * FROM frames_fts WHERE frames_fts MATCH ?").all('app_name:"zoom.us"');
    console.log("bound:", rows);
} catch (e) { console.log("bound err:", e.message); }
