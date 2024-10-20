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
            // "ID: {}, ISBN: {}, Nome: {}, Nome do Autor: {}",
            "\nISBN: {}, \nNome: {}, \nNome do Autor: {}",
            self.isbn, self.nome, self.nomeautor
        )
    }
}

pub fn cadastrar_livro(isbn: String, nome: String, nomeautor: String) -> Result<Livro, String> {
    let dir = "livros.json";
    let mut open_options = OpenOptions::new();
    let create = open_options.read(true).write(true).create(true);

    let mut file = create
        .open(dir)
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo"))?;

    let mut livros: Vec<Livro> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    // Verifica se o livro com o mesmo ISBN já existe
    if livros.iter().any(|l| l.isbn == isbn) {
        return Err(String::from("Livro com o mesmo ISBN já cadastrado."));
    }

    let livro = Livro::new(isbn, nome, nomeautor);

    livros.push(livro.clone());
    let livros_json =
        serde_json::to_string(&livros).map_err(|_| String::from("Erro ao serializar os livros"))?;

    file.set_len(0)
        .map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.write_all(livros_json.as_bytes())
        .map_err(|_| String::from("Erro ao escrever no arquivo"))?;
    file.flush()
        .map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;

    Ok(livro)
}

pub fn buscar_livro(isbn: String) -> Result<Livro, String> {
    let dir = "livro.json";

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false) // Não cria o arquivo se não existir
        .open(dir)
        .map_err(|_| String::from("Erro ao abrir o arquivo pessoas.json"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo pessoas.json"))?;

    // Remover espaços em branco desnecessários
    let conteudos = conteudos.trim();

    // Verificar o conteúdo lido do arquivo
    println!("Conteúdo lido do arquivo: {}", conteudos);

    let livros: Vec<Livro> = serde_json::from_str(conteudos)
        .map_err(|_| String::from("Erro ao deserializar as pessoas"))?;

    for livro in livros {
        if livro.isbn == isbn {
            return Ok(livro);
        }
    }

    Err(String::from("Livro não encontrado"))
}

pub fn listar_livros_disponiveis() {
    let dir = "livros.json"; 

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false) 
        .open(dir)
        .expect("Erro ao abrir o arquivo livros.json");

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .expect("Erro ao ler o arquivo livros.json");

    // Remover espaços em branco desnecessários
    let conteudos = conteudos.trim();

    // Verificar o conteúdo lido do arquivo
   // println!("Conteúdo lido do arquivo: {}", conteudos);

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
