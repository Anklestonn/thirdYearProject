use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufRead, Write};

pub fn write_already_targeted(already_targeted: Vec<String>){

    let contents = make_one_string(already_targeted);
    
    let order_option = OpenOptions::new().create(true).write(true).create(true).truncate(true).open("conf/".to_owned() + "used_ip");
    let mut order = match order_option {
        Ok(file) => file,
        Err(_error) => {
            //dbg!(error);
            //println!("impossible to write contents");
            return ();
        },
    };

    let contents_bytes = contents.as_bytes();
    let _ = order.write_all(&contents_bytes);

}

pub fn pop_first_line_target() -> Vec<String> {
    let f = File::open("conf/ip_victims");
    
    let f = match f{
        Ok(file) => file,
        Err(..) => return vec!(),
    };

    let file = BufReader::new(f);

    
    let mut contents: Vec <_> = file
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if contents.len() > 0 {

        contents.remove(0);
    }
    contents

}

pub fn write_rest_to_hack(rest_vec: Vec<String>) {

    let contents = make_one_string(rest_vec);

    let order_option = OpenOptions::new().create(true).write(true).truncate(true).open("www/".to_owned() + "ip_victims");

    
    let mut order = match order_option {
        Ok(file) => file,
        Err(_error) => return (),
    };


    let contents_bytes = contents.as_bytes();
    let _ = order.write_all(&contents_bytes);

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
