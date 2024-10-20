use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)] 
pub struct Livro {
    pub id: Uuid,
    pub isbn: String,
    pub nome: String,
    pub nomeautor: String,
}

impl Livro {
    pub fn new(isbn: String, nome: String, nomeautor: String) -> Self {
        Livro {
            id: Uuid::new_v4(),
            isbn,
            nome,
            nomeautor,
        }
    }
}

pub trait Listar {
    fn listar_struct(&self) -> String;
}

impl Listar for Livro {
    fn listar_struct(&self) -> String {
        format!(
            "ID: {}, ISBN: {}, Nome: {}, Nome do Autor: {}",
            self.id, self.isbn, self.nome, self.nomeautor
        )
    }
}

pub fn cadastrar_livro(isbn: String, nome: String, nomeautor: String) -> Result<Livro, String> {
    let dir = "livros.json";
    let mut open_options = OpenOptions::new();
    let create = open_options.read(true).write(true).create(true);

    let mut file = create.open(dir)
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos).map_err(|_| String::from("Erro ao ler o arquivo"))?;

    let mut livros: Vec<Livro> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    let livro = Livro::new(isbn, nome, nomeautor);

    livros.push(livro.clone());
    let livros_json = serde_json::to_string(&livros).map_err(|_| String::from("Erro ao serializar os livros"))?;

    file.set_len(0).map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.write_all(livros_json.as_bytes()).map_err(|_| String::from("Erro ao escrever no arquivo"))?;

    Ok(livro)
}
