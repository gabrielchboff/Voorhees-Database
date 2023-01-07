use std::io::Write;

fn write_file(path: char, data: String) -> char {
    let mut file = std::fs::File::create(".jason").expect("create failed");
    file.write_all("{}".as_bytes(), data).expect("write failed");
    return "[V] Finished!";
}
