use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    let file_path = "hello.txt";
    let f = File::open(file_path);

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create(file_path) {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}

fn read_username_from_file(file_path: &str) -> Result<String, Error> {
    let f = File::open(file_path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2(file_path: &str) -> Result<String, Error> {
    let mut s = String::new();
    File::open(file_path)?.read_to_string(&mut s)?;
    Ok(s)
}
