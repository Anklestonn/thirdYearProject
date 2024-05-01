

use std::fs;
use std::io::prelude::*;

pub fn get_param(param: &str) -> Vec<u8> {
    // param have to be controlled before, of else there is an IDOR vulnerability.
    let content_option = fs::OpenOptions::new().read(true).open("conf/".to_owned() + param);
    let mut content = match content_option {
        Ok(file) => file,
        Err(..) => {
            println!("warning: No param to send");
            return Vec::new();
        },
    };

    let mut content_vec: Vec<u8> = Vec::new();
    let _ = content.read_to_end(&mut content_vec);
    return content_vec;

}


