use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use crate::models::livro::Livro;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pessoa {
    pub id: Uuid,
    pub nome: String,
    pub cpf: String,
    pub livros_pendentes: Vec<Livro>,
}

impl Pessoa {
    pub fn new(nome: String, cpf: String) -> Self {
        Pessoa {
            id: Uuid::new_v4(),
            nome,
            cpf,
            livros_pendentes: Vec::new(), // Inicializa o vetor corretamente
        }
    }
}
pub fn cadastrar_pessoa(nome_pessoa: String, cpf_pessoa: String, livros: Vec<Livro>) -> Result<Pessoa, String> {
    let dir = "pessoas.json";
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(dir)
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos).map_err(|_| String::from("Erro ao ler o arquivo"))?;

    let mut pessoas: Vec<Pessoa> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    if pessoas.iter().any(|p| p.cpf == cpf_pessoa) {
        return Err(String::from("Pessoa com esse CPF já cadastrada"));
    }

    let pessoa = Pessoa {
        id: Uuid::new_v4(),
        nome: nome_pessoa,
        cpf: cpf_pessoa,
        livros_pendentes: livros,
    };

    pessoas.push(pessoa.clone());
    let pessoas_json = serde_json::to_string(&pessoas).map_err(|_| String::from("Erro ao serializar as pessoas"))?;

    file.set_len(0).map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.write_all(pessoas_json.as_bytes()).map_err(|_| String::from("Erro ao escrever no arquivo"))?;

    Ok(pessoa)
}
