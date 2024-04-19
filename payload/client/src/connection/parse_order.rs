

pub fn get_to_download(order: Vec<String>) -> Vec<String> {
    // TODO
    // get order's file contents,
    // each item of order correspond at one line.
    //
    // If one line began by "Download " add to the Vec returns the file name.
    //
    // Example contents an order file can be :
    // ```
    // Download server_binary
    // Download installation_launch.sh
    // Exec installation_launch.sh Bash
    // ```
    //
    // in this case, the vector returned will be ["server_binary", "installation_launch.sh"]
    
    let mut returns: Vec<String> = Vec::new();

    returns.append(&mut vec!["installation.sh".to_owned()]); // TODO: Replace this line by the real code.
    return returns;

}


