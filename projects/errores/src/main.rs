use std::{
    fs::File,
    fs,
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the fiel: {:?}", other_error);
            }
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut usernema_file = File::open("hello.txt")?;// el ? devuelve Ok o Err
    let mut username = String::new();
    usernema_file.read_to_string(&mut username)?;
    Ok(username)
}

fn mas_short_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn la_mas_corta() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}