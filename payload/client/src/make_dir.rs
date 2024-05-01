
use std::fs::create_dir;
use std::io::ErrorKind;

pub fn make_dir() -> i32 {
    // check if directory ../www, ../downloaded, ../conf exists, if no, create them. 

    let mut exit_code = 0;
    exit_code += create_a_dir("www");
    exit_code += create_a_dir("downloaded");
    exit_code += create_a_dir("conf");

    return exit_code;
}

fn create_a_dir(dir: &str) -> i32 {
    let res = create_dir(dir);
    match res {
        Ok(_val) => return 0,
        Err(ee) => {
            if ee.kind() == ErrorKind::AlreadyExists {
                return 0
            } else {
                println!("fail: {}", ee.kind());
                return 1;
            }
        },
    };

}
