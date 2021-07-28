use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let filename = "nyanpasu.txt";
    let f = File:: open(filename);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(created) => created,
                Err(e) => panic!("Problem creating {}: {:?}", filename, e)
            },
            other_error => {
                panic!("Problem opening {}: {:?}", filename, other_error)
            }
        }
    };
}
