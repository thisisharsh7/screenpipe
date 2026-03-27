use std::process::Command;

fn is_macos_admin() -> bool {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("id").arg("-Gn").output() {
            if let Ok(groups) = String::from_utf8(output.stdout) {
                return groups.split_whitespace().any(|g| g == "admin");
            }
        }
        false
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

fn main() {
    println!("Is admin? {}", is_macos_admin());
}
