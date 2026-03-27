fn main() {
    #[cfg(target_os = "linux")]
    {
        let lib1 = unsafe { libloading::Library::new("libayatana-appindicator3.so.1") };
        println!("lib1: {}", lib1.is_ok());
    }
}
