use crate::models::livro::{Listar, Livro};
use crate::models::pessoa::Pessoa;
use crate::models::multa::Multa;
use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use uuid::Uuid;

use super::livro::cadastrar_livro;
use super::multa::cadastrarmulta;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Emprestimo {
    pub id: Uuid,
    pub livro: Livro,
    pub data_emprestimo: DateTime<Utc>,
    pub data_devolucao: DateTime<Utc>,
    pub pessoa: Pessoa,
}

impl Emprestimo {
    pub fn new(livro: Livro, pessoa: Pessoa) -> Self {
        Emprestimo {
            id: Uuid::new_v4(),
            livro,
            data_emprestimo: Utc::now(),
            data_devolucao: Utc::now() + chrono::Duration::seconds(10),
            pessoa,
        }
    }
}

pub fn emprestar(pessoa: Pessoa, livro: Livro) -> Result<Emprestimo, String> {
    //let dir_livro = "livros.json";
    let mut open_options = OpenOptions::new();
    let create = open_options.read(true).write(false).create(false);

    let mut file_livro = create
        .open("livros.json")
        .map_err(|_| String::from("Erro ao abrir o arquivo de livros"))?;

    let mut conteudos_livro = String::new();
    file_livro
        .read_to_string(&mut conteudos_livro)
        .map_err(|_| String::from("Erro ao ler o arquivo de livros"))?;

    let mut livros: Vec<Livro> = serde_json::from_str(&conteudos_livro).unwrap_or_else(|_| vec![]);

    //let dir_pessoa = "pessoas.json";
    let mut file_pessoa = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open("pessoas.json")
        .map_err(|_| String::from("Erro ao abrir o arquivo de pessoas"))?;

    let mut conteudos_pessoa = String::new();
    file_pessoa
        .read_to_string(&mut conteudos_pessoa)
        .map_err(|_| String::from("Erro ao ler o arquivo de pessoas"))?;

    let mut pessoas: Vec<Pessoa> =
        serde_json::from_str(&conteudos_pessoa).unwrap_or_else(|_| vec![]);

    // Verifica se a pessoa está registrada
    if let Some(_pos_pessoa) = pessoas.iter_mut().find(|p| p.id == pessoa.id) {
        if let Some(pos_livro) = livros.iter().position(|l| *l == livro) {
            // Remover o livro da lista de livros
            livros.remove(pos_livro);

            // Atualizar o arquivo de livros
            let livros_atualizados = serde_json::to_string_pretty(&livros)
                .map_err(|_| String::from("Erro ao serializar os livros"))?;

            let mut file_livro = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("livros.json")
                .map_err(|_| String::from("Erro ao abrir o arquivo de livros para escrita"))?;

            file_livro
                .write_all(livros_atualizados.as_bytes())
                .map_err(|_| String::from("Erro ao escrever no arquivo de livros"))?;

            // Registrar o empréstimo
            let dir = "emprestimos.json";
            let mut file_emprestimo = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(dir)
                .map_err(|_| String::from("Erro ao abrir o arquivo de empréstimos"))?;

            let mut conteudos = String::new();
            file_emprestimo
                .read_to_string(&mut conteudos)
                .map_err(|_| String::from("Erro ao ler o arquivo de empréstimos"))?;

            let mut emprestimos: Vec<Emprestimo> =
                serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

            let novo_emprestimo = Emprestimo::new(livro.clone(), pessoa.clone());

            emprestimos.push(novo_emprestimo.clone());

            let emprestimos_json = serde_json::to_string(&emprestimos)
                .map_err(|_| String::from("Erro ao serializar os empréstimos"))?;

            file_emprestimo
                .set_len(0)
                .map_err(|_| String::from("Erro ao limpar o arquivo de empréstimos"))?;
            file_emprestimo
                .seek(SeekFrom::Start(0))
                .map_err(|_| String::from("Erro ao reposicionar o cursor no arquivo"))?;
            file_emprestimo
                .write_all(emprestimos_json.as_bytes())
                .map_err(|_| String::from("Erro ao escrever no arquivo de empréstimos"))?;
            file_emprestimo
                .flush()
                .map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;

            Ok(novo_emprestimo)
        } else {
            return Err(String::from("Livro não encontrado ou indisponível"));
        }
    } else {
        return Err(String::from(
            "Pessoa não encontrada... tente cadastrar a pessoa antes de realizar empréstimo",
        ));
    }
}

pub fn devolver(pessoa: Pessoa, isbn: String) -> Result<(), String> {
    // Remover da lista de empréstimos
    let dir = "emprestimos.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(dir)
        .map_err(|_| String::from("Erro ao abrir o arquivo de empréstimos"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo de empréstimos"))?;
    let mut emprestimos: Vec<Emprestimo> =
        serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    if let Some(pos) = emprestimos
        .iter()
        .position(|e| e.livro.isbn == isbn && e.pessoa.id == pessoa.id)
    {
        // Adiciona o livro que está sendo devolvido à lista de livros
        let livro_devolvido = &emprestimos[pos].livro;
        cadastrar_livro(
            livro_devolvido.isbn.clone(),
            livro_devolvido.nome.clone(),
            livro_devolvido.nomeautor.clone(),
            livro_devolvido.ano.clone(),    
        )?;
        let emprestimo_devolvido = emprestimos.remove(pos);


        let emprestimos_json = serde_json::to_string(&emprestimos)
            .map_err(|_| String::from("Erro ao serializar os empréstimos"))?;
        file.set_len(0)
            .map_err(|_| String::from("Erro ao limpar o arquivo de empréstimos"))?;
        file.write_all(emprestimos_json.as_bytes())
            .map_err(|_| String::from("Erro ao escrever no arquivo de empréstimos"))?;
        file.flush()
            .map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;
        
        let mut data_atual = Utc::now();
        let mut taxa: f64 = 2.0;
        if emprestimo_devolvido.data_devolucao <= data_atual{
            println!("Empréstimo devolvido com atraso!");
            println!("Data de devolução: {}", data_atual.format("%d/%m/%Y %H:%M:%S"));
            cadastrarmulta(pessoa, taxa);
        }

        Ok(())
    } else {
        Err(String::from("Empréstimo não encontrado"))
    }
}
pub fn listar_emprestados() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("emprestimos.json")
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
        println!("Nenhum livro disponível.");
        return;
    }

    let emprestimos: Vec<Emprestimo> = serde_json::from_str(&conteudos).unwrap_or_else(|_| vec![]);

    for e in emprestimos {
        let info = e.livro.listar_struct();
        print!("{}", info);
    }
}
