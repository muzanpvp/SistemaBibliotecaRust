use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::OpenOptions;
use crate::models::pessoa::Pessoa;
use crate::models::livro::Livro;
use std::io::{Read, Write};
use chrono::{DateTime, Local, Utc};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Emprestimo {
    pub id: Uuid,
    pub livro: Livro,
    pub data_emprestimo: DateTime<Utc>,
    pub data_devolucao: DateTime<Utc>,
    pub pessoa: Pessoa,
    pub status: bool,
}

impl Emprestimo {
    pub fn new(livro: Livro, pessoa: Pessoa, status: bool) -> Self {
        Emprestimo {
            id: Uuid::new_v4(),
            livro,
            data_emprestimo: Utc::now(),
            data_devolucao: Utc::now(),   
            pessoa,
            status,
        }
    }
}
//To do atulizar a lista de livros pendentes da pessoa
//Prestar na data e hora...
pub fn emprestar(pessoa: Pessoa, livro: Livro) -> Result<Emprestimo, String> {
    let dir = "emprestimo.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(dir)
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos).map_err(|_| String::from("Erro ao ler o arquivo"))?;

    let mut emprestimos: Vec<Emprestimo> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    if emprestimos.iter().any(|e| e.pessoa == pessoa && e.livro == livro) {
        return Err(String::from("Essa pessoa já está com este livro emprestado"));
    }

    let novo_emprestimo = Emprestimo::new(livro.clone(), pessoa.clone(), true); // Cria um novo empréstimo

    emprestimos.push(novo_emprestimo.clone()); 

    let emprestimos_json = serde_json::to_string(&emprestimos).map_err(|_| String::from("Erro ao serializar os empréstimos"))?;

    file.set_len(0).map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.write_all(emprestimos_json.as_bytes()).map_err(|_| String::from("Erro ao escrever no arquivo"))?;

    Ok(novo_emprestimo) 
}
