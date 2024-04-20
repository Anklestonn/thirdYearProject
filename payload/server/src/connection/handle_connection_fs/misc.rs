
use std::fs::read;


pub fn fs_read_file(file: String) -> Vec<u8>{

    match read("../www/".to_owned() + &file) {
        Ok(f) => return f,
        Err(..) => {
            println!("Error reading file (fs)");
            return "404".as_bytes().to_vec();
        },
    };
}


pub fn parse_file_name(lines: Vec<String>) -> String {
    let first_line: &String = &lines[0];

    for c in first_line.chars() {
        match c {
            ' ' | '/' | '\\' | '`' | '|' | '\'' => return "error_file".to_owned(),
            _other => {}
        };
    }
    println!("sent file (fs): {}",first_line);
    return first_line.clone()
}
