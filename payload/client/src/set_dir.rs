

use std::env;
use std::path::Path;
use std::env::set_current_dir;
use std::process::exit;

pub fn set_working_directory() {
    let prog_path_raw = env::args().next();
    let prog_path = match prog_path_raw {
        Some(string) => string,
        None => exit(1),
    };
    let path = Path::new(&prog_path).parent().unwrap();
    if ! set_current_dir(path).is_ok() {
        //println!("Error setting the path");
        exit(1);
    }
    //println!("prog launched at: {}", prog_path);
}

