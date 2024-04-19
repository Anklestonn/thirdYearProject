use std::fs;
use std::io::Write;

pub fn download(downFile: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Download the file 'file' from the server port 7870 and put it in the working directory.
    // If the file already exists, remove it.
    //will add remove code to main excute then here?:
    



    //url to download the file from port 7870
    let url = format!("http://{}:7870{}",downFile);
    //download file using reqwest
    match reqwest::blocking::get(&url){
        //if successful
        Ok(response) => {
            //checking if so 
            if response.status().is_success() {
                //overwrite contents in the download file in working directory
                let mut file_buff = fs::File::overwrite(downFile)?
                //get the content of the response
                let mut content = response.bytes()?
                //write download to the file
                file_buff.write_all(&content)?;

                println!("file '{}' downloaded successfully", downFile);
                Ok(())
            }
            else{
                println("failed download ", downFile, response.status());
                Err("download failed".into())
            }
}

