fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());

    println!(r#"cargo:rustc-cfg=feature="pass""#);
}
