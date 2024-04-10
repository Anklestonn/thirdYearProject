
use std::fs;
use std::io::prelude::*;

pub fn get_order() -> Vec<u8> {
    let content_option = fs::OpenOptions::new().read(true).open("order");
    let mut content = match content_option {
        Ok(file) => file,
        Err(..) => {
            println!("warning: No order to send");
            return Vec::new();
        },
    };

    let mut content_vec: Vec<u8> = Vec::new();
    let _ = content.read_to_end(&mut content_vec);
    return content_vec;

}
