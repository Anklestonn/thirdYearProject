

#[derive(Debug)]
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

        return 0;
    }

    pub fn get_binscript(&mut self) -> &String {
        &self.binscript
    }
}
