

mod binprog;

//#[allow(unused_variables)]
pub fn get_to_download(order: Vec<String>) -> Vec<String> {
    // TODO
    // creating vector to store file names in download
    let mut downloads: Vec<String> = Vec::new();


    // go over each line in order
    for line in order {
        //checking line for "download"
        if line.starts_with("download ") {
            //if yes, extract file name and add to downloads vector 
            let file_name = line.trim_start_matches("download ").to_owned();
            downloads.push(file_name);
        }
    }

    downloads
}

    //let mut returns: Vec<String> = Vec::new();

    //returns.append(&mut vec!["installation.sh".to_owned()]); // TODO: Replace this line by the real code.
    //return returns;



//#[allow(unused_variables)]
pub fn get_to_exec(order: Vec<String>) -> Vec<binprog::Binprog> {
    // TODO
    //vector to store binprog objects
    let mut execs: Vec<binprog::Binprog> = Vec::new();

    //go over eavh line
    for line in order {
        //check if starts with "exec"
        if line.starts_with("exec ") {
            //spilt line into sections
            let parts: Vec<&str> = line.trim_start_matches("exec ").split_whitespace().collect();
            //extract program name
            let program_name = parts[0].to_owned();
            //extract the arguments. if any 
            let args = if parts.len() > 1 { Some(parts[1].to_owned()) } else { None };
            //create binprog object with the program name and arguments, and add it to the execs
            //vector
            execs.push(binprog::Binprog::new(program_name, args));
        }
    }

    execs
}

    // in this case, the vector returned will be [Binprog<"installation.sh", Some("bash")>, Binprog<"installation.sh", None>]


    //return vec![binprog::Binprog::new("installation.sh".to_owned(), Some("sh".to_owned()))]

