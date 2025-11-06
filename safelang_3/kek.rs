fn main() {
    let mut out = String::new();

    for entry in r#std::fs::read_dir("/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let s = path.to_string_lossy().to_string();
        if s.starts_with("/flag-") && s.ends_with(".txt") {
            out = r#std::fs::read_to_string(path).unwrap();
            break;
        }
    }

    print(out);
}

// base64 -w0 kek.rs