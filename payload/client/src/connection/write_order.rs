

use std::io::Write;
use std::fs;

pub fn write_order(order_name: &str, contents_vec: Vec<String>) -> u32{
    // return an error code of 1 in case file couldn't be write.
    // if everything good, return 0.
    
    let contents = make_one_string(contents_vec);
    
    let order_option = fs::OpenOptions::new().write(true).create(true).truncate(true).open("conf/".to_owned() + order_name);
    let mut order = match order_option {
        Ok(file) => file,
        Err(_error) => {
            //dbg!(error);
            //println!("impossible to write contents");
            return 1;
        },
    };

    let contents_bytes = contents.as_bytes();
    let _ = order.write_all(&contents_bytes);

    return 0;
}


fn make_one_string(vec_string: Vec<String>) -> String {
    let mut ret = String::new();
    for chaine in vec_string.iter() {
        if chaine != "" {
            ret.push_str(&chaine);
            ret.push('\n');
        }

    }

    return ret
}
