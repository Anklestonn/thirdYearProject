

use std::io::Write;
use std::fs;

pub fn write_order(contents: String) -> u32{
    // return an error code of 1 in case file couldn't be write.
    // if everything good, return 0.
    
    let order_option = fs::OpenOptions::new().write(true).create(true).open("order");
    let mut order = match order_option {
        Ok(file) => file,
        Err(error) => {
            dbg!(error);
            println!("impossible to write contents");
            return 1;
        },
    };

    let contents_bytes = contents.as_bytes();
    let _ = order.write_all(&contents_bytes);

    return 0;

    
    
}
