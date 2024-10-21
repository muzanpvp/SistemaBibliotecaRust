use crate::models::pessoa::Pessoa;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Multa {
    pub id: Uuid,
    pub pessoa: Pessoa,
    pub valor: f64,
    pub status: String,
}

impl Multa {
    pub fn new( pessoa: Pessoa, valor: f64, status: String) -> Self {
        Multa {
            id: Uuid::new_v4(),
            pessoa,
            valor,
            status,
        }
    }
}

pub trait Listar {
    fn listar_struct(&self) -> String;
}

impl Listar for Multa {
    fn listar_struct(&self) -> String {
        format!(
            "\nPessoa: {}, \nValor: {},\nStatus: {}\n",
            self.pessoa.nome, self.valor, self.status
        )
    }
}

pub fn cadastrarmulta(pessoa: Pessoa,valor: f64) -> Result<Multa, String> {

let mut open_options = OpenOptions::new();
    let create = open_options.read(true).write(true).create(true);
    let mut file = create
        .open("multas.json")
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo"))?;
    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo"))?;
    let mut multas: Vec<Multa> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    let status:String = String::from("Pendente");
    let multa: Multa = Multa::new(pessoa, valor, status);
    multas.push(multa.clone());

    let multas_json =
        serde_json::to_string(&multas).map_err(|_| String::from("Erro ao serializar os multas"))?;
    file.set_len(0)
        .map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|_| String::from("Erro ao reposicionar o cursor no arquivo"))?;
    file.write_all(multas_json.as_bytes())
        .map_err(|_| String::from("Erro ao escrever no arquivo"))?;
    file.flush()
        .map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;
    Ok(multa)
}

pub fn listarmultas(){
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("multas.json")
        .map_err(|_| String::from("Erro ao abrir o arquivo de empréstimos"));

        let mut file = match file {
            Ok(file) => file,
            Err(_) => {
                println!("Arquivo de livros não encontrado.");
                return;
            }
        };

    let mut conteudos = String::new();
    file
        .read_to_string(&mut conteudos)
        .expect("Erro ao ler o arquivo de empréstimos");

    if conteudos.is_empty() {
        println!("Nenhuma multa disponível.");
        return;
    }

    let multas: Vec<Multa> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    for m in multas {
        let info = m.listar_struct();
        print!("{}", info);
    }
}