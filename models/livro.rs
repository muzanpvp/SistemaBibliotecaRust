use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom};
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
            "\nISBN: {}, \nNome: {}, \nNome do Autor: {}",
            self.isbn, self.nome, self.nomeautor
        )
    }
}

pub fn cadastrar_livro(isbn: String, nome: String, nomeautor: String) -> Result<Livro, String> {
    let mut open_options = OpenOptions::new();
    let create = open_options.read(true).write(true).create(true);
    let mut file = create
        .open("livros.json")
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo"))?;
    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo"))?;
    let mut livros: Vec<Livro> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    let livro = Livro::new(isbn, nome, nomeautor);
    livros.push(livro.clone());
    
    for l in &livros {
        if livro.isbn == l.isbn && (livro.nome != l.nome || livro.nomeautor != l.nomeautor) {
            return Err(String::from("Não foi possível realizar o cadastro... confira o ISBN"));
        }
    }
    

    let livros_json = serde_json::to_string(&livros).map_err(|_| String::from("Erro ao serializar os livros"))?;
    file.set_len(0).map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.seek(SeekFrom::Start(0)).map_err(|_| String::from("Erro ao reposicionar o cursor no arquivo"))?;
    file.write_all(livros_json.as_bytes())
        .map_err(|_| String::from("Erro ao escrever no arquivo"))?;
    file.flush().map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;
    Ok(livro)
}

pub fn buscar_livro(isbn: String) -> Result<Livro, String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false) // Não cria o arquivo se não existir
        .open("livros.json")
        .map_err(|_| String::from("Erro ao abrir o arquivo livros.json"))?;
    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo livros.json"))?;
    
    let conteudos = conteudos.trim();
    println!("Conteúdo lido do arquivo: {}", conteudos);
    let livros: Vec<Livro> = serde_json::from_str(conteudos)
        .map_err(|_| String::from("Erro ao deserializar os livros"))?;
    for livro in livros {
        if livro.isbn == isbn {
            return Ok(livro);
        }
    }
    Err(String::from("Livro não encontrado"))
}

pub fn listar_livros_disponiveis() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open("livros.json")
        .expect("Erro ao abrir o arquivo livros.json");
    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .expect("Erro ao ler o arquivo livros.json");

    let conteudos = conteudos.trim();
    if conteudos.is_empty() {
        println!("Nenhum livro disponível.");
        return;
    }
    let livros: Vec<Livro> = serde_json::from_str(conteudos)
        .expect("Erro ao deserializar os livros");
    for livro in livros {
        let info = livro.listar_struct();
        println!("{}", info);
    }
}
