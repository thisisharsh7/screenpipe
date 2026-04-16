fn main() {
    let msg = "test".to_string();
    let e = sqlx::Error::Protocol(msg);
}