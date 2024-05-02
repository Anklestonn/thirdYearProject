
use std::process::Command;

#[derive(Debug)]
#[derive(Clone)]
#[allow(dead_code)]
pub struct Binprog {
    binscript: String,
    program: Option<String>,
}

impl Binprog {
    pub fn new(bin: String, prog: Option<String>) -> Binprog {
        Binprog {
            binscript: bin,
            program: prog,
        }
    }

    pub fn exec(&mut self) -> u32 {
        // execute the programme.
        
        let dir = "downloaded/".to_owned();
        
        match &self.program {
            Some(thing) => {
                let status = Command::new(thing).arg( dir + &self.binscript.clone()).status();
                match status {
                    Ok(s) => {
                        if s.success() {
                            return 0
                        } else {
                            return 1
                        }
                    },
                    Err(..) => return 1
                };
            },
            None => {
                let status = Command::new( dir + &self.binscript.clone()).status();
                match status {
                    Ok(s) => {
                        if s.success() {
                            return 0
                        } else {
                            return 1
                        }
                    },
                    Err(..) => return 1
                };

            },
        };
    }
    #[allow(dead_code)]
    pub fn get_binscript(&mut self) -> &String {
        &self.binscript
    }
}
