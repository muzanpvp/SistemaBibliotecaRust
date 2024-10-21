use crate::models::pessoa::Pessoa;
use crate::models::multa::{self, Multa};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Pagamento {
    pub id: Uuid,
    pub data_devolucao: DateTime<Utc>,
    pub pessoa: Pessoa,
    pub valor: f64,
    pub status: String,
}

impl Pagamento {
    pub fn new( pessoa: Pessoa, valor: f64, status: String) -> Self {
        Pagamento {
            id: Uuid::new_v4(),
            data_devolucao: Utc::now() + chrono::Duration::days(10),
            pessoa,
            valor,
            status,
        }
    }
}

pub trait Listar {
    fn listar_struct(&self) -> String;
}

impl Listar for Pagamento {
    fn listar_struct(&self) -> String {
        format!(
            "\nNome: {}, \nValor: {}, \nStatus do pagamento: {},\n",
            self.pessoa.nome, self.valor, self.status
        )
    }
}


pub fn cadastrar_pagamento(pessoa: Pessoa,valor: f64) -> Result<Pagamento, String> {
     let mut file_pagamento = OpenOptions::new()
                 .read(true)
                 .write(true)
                 .truncate(true)
                 .create(true)
                 .open("pagamentos.json")
                 .map_err(|_| String::from("Erro ao abrir o arquivo de pagamentos para escrita"))?;
 
     let mut conteudos_pagamento = String::new();
     file_pagamento
         .read_to_string(&mut conteudos_pagamento)
         .map_err(|_| String::from("Erro ao ler o arquivo de pagamentos"))?;
 
     let mut pagamentos: Vec<Pagamento> = serde_json::from_str(&conteudos_pagamento).unwrap_or_else(|_| vec![]);
    
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
             // Atualizar o arquivo de pagamentos
             let pagamentos_atualizados = serde_json::to_string_pretty(&pagamentos)
                 .map_err(|_| String::from("Erro ao serializar os pagamentos"))?;
 
             // Registrar o pagamento
 
             let status = String::from("Confirmado");
             let novo_pagamento = Pagamento::new(pessoa.clone(), valor, status.clone());
 
             pagamentos.push(novo_pagamento.clone());
 
             let pagamentos_json = serde_json::to_string(&pagamentos)
                 .map_err(|_| String::from("Erro ao serializar os pagamentos"))?;
 
             file_pagamento
                 .set_len(0)
                 .map_err(|_| String::from("Erro ao limpar o arquivo de pagamentos"))?;
             file_pagamento
                 .seek(SeekFrom::Start(0))
                 .map_err(|_| String::from("Erro ao reposicionar o cursor no arquivo"))?;
             file_pagamento
                 .write_all(pagamentos_json.as_bytes())
                 .map_err(|_| String::from("Erro ao escrever no arquivo de pagamentos"))?;
             file_pagamento
                 .flush()
                 .map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;
 
                 let dir_multas = "multas.json";
                 let mut file_multa = OpenOptions::new()
                     .read(true)
                     .write(true)
                     .create(true)
                     .open(dir_multas)
                     .map_err(|_| String::from("Erro ao abrir o arquivo de multas"))?;
             
                 let mut conteudos_multas = String::new();
                 file_multa.read_to_string(&mut conteudos_multas)
                     .map_err(|_| String::from("Erro ao ler o arquivo de multas"))?;
                 let mut multas: Vec<Multa> =
                     serde_json::from_str(&conteudos_multas).unwrap_or_else(|_| vec![]);
             
                 if let Some(pos) = multas
                  .iter()
                  .position(|m| m.pessoa.id == pessoa.id){          
                    println!("Multa paga com sucesso de {}", pessoa.nome);
                    multas.remove(pos);
                  } 

                  let multas_json =
                  serde_json::to_string(&multas).map_err(|_| String::from("Erro ao serializar os multas"))?;
              file_multa.set_len(0)
                  .map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
                file_multa.seek(SeekFrom::Start(0))
                  .map_err(|_| String::from("Erro ao reposicionar o cursor no arquivo"))?;
                file_multa.write_all(multas_json.as_bytes())
                  .map_err(|_| String::from("Erro ao escrever no arquivo"))?;
                file_multa.flush()
                  .map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?;
              



             Ok(novo_pagamento)
     } else {
         return Err(String::from(
             "Pessoa não encontrada... tente cadastrar a pessoa antes de realizar pagamento",
         ));
     }
}

pub fn listarpagamentos(){
    let file = OpenOptions::new()
    .read(true)
    .write(false)
    .create(false)
    .open("pagamentos.json");

let mut file = match file {
    Ok(file) => file,
    Err(_) => {
        println!("Arquivo de pagamentos não encontrado.");
        return;
    }
};

let mut conteudos = String::new();
file.read_to_string(&mut conteudos)
    .expect("Erro ao ler o arquivo pagamentos.json");

let conteudos = conteudos.trim();
if conteudos.is_empty() {
    println!("Nenhum pagamento disponível.");
    return;
}
let pagamentos: Vec<Pagamento> =
    serde_json::from_str(conteudos).expect("Erro ao deserializar os pagamentos");

println!("Pagamentos:");
for pagamento in pagamentos {
       let info = pagamento.listar_struct();
       println!("{}",info);
}
}