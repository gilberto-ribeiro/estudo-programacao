use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
    error::Error,
};

use error_handling::guessing_game::*;

fn main() {
    let guess: Guess = Guess::new(101);
    println!("Chute é {}", guess.value());
}

fn erro_0() {
    let nome_do_arquivo: &str = "arquivo_0.txt";
    let _greeting_file = match File::open(nome_do_arquivo) {
        Ok(file) => file,
        Err(erro) => panic!("Erro ao abrir o arquivo {nome_do_arquivo}: {erro}"),
    };
}

fn erro_1() {
    let nome_do_arquivo: &str = "arquivo_novo.txt";
    let greeting_file_result = File::open(nome_do_arquivo);
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(nome_do_arquivo) {
                Ok(new_file) => new_file,
                Err(error_create) => panic!("Erro ao criar um novo arquivo. Erro: {error_create}"),
            },
            other_error => panic!("Erro ao abrir o arquivo: Erro: {other_error}"),
        },
    };
}

fn erro_2() {
    let file_name: &str = "file_2.txt";
    let _file = File::open(file_name).unwrap();
}

fn erro_3() {
    let file_name: &str = "file_3.txt";
    let _file: File =
        File::open(file_name).expect(format!("Erro ao abrir o arquivo {file_name}").as_str());
}

fn read_username_from_file_0() -> Result<String, io::Error> {
    let username_file_result: Result<File, io::Error> = File::open("hello.txt");
    let mut username_file: File = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut username: String = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}