#[derive(Debug)]
pub struct Loxerror {
    line: usize,
    message: String
}

impl Loxerror {
    pub fn error(line:usize, message:String) -> Loxerror {
        Loxerror { line, message }
    }

    pub fn report(&self, loc:String) {
        eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
    }
}