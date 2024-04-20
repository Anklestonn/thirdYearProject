

mod binprog;

#[allow(unused_variables)]
pub fn get_to_download(order: Vec<String>) -> Vec<String> {
    // TODO
    // order correspond to a file contents,
    // each item of order correspond at one line.
    //
    // If one line began by "download " add to the Vec returns the file name.
    //
    // Example contents an order file can be :
    // ```
    // download server_binary
    // download installation.sh
    // exec installation.sh bash
    // exec server_binary
    // ```
    //
    // in this case, the vector returned will be ["server_binary", "installation.sh"]
    
    let mut returns: Vec<String> = Vec::new();

    returns.append(&mut vec!["installation.sh".to_owned()]); // TODO: Replace this line by the real code.
    return returns;

}

#[allow(unused_variables)]
pub fn get_to_exec(order: Vec<String>) -> Vec<binprog::Binprog> {
    // TODO
    // order correspond to a file contents,
    // each item of order correspond at one line.
    //
    // If one line began by "download " add to the Vec returns the file name.
    //
    // Example contents an order file can be :
    // ```
    // download server_binary
    // download installation.sh
    // exec installation.sh bash
    // exec server_binary
    // ```
    //
    // in this case, the vector returned will be [Binprog<"installation.sh", Some("bash")>, Binprog<"installation.sh", None>]


    return vec![binprog::Binprog::new("installation.sh".to_owned(), Some("bash".to_owned()))]
}



