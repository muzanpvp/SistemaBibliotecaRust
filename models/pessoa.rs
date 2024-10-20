use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)] 
pub struct Pessoa {
    pub id: Uuid,
    pub nome: String,
    pub cpf: String,
}

/*impl Pessoa {
    pub fn new(nome: String, cpf: String) -> Self {
        Pessoa {
            id: Uuid::new_v4(),
            nome,
            cpf, 
        }
    }
}*/

pub fn cadastrar_pessoa(nome_pessoa: String, cpf_pessoa: String) -> Result<Pessoa, String> {
   // let dir = "pessoas.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Cria o arquivo se não existir
        .open("pessoas.json")
        .map_err(|_| String::from("Erro ao abrir ou criar o arquivo pessoas.json"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos).map_err(|_| String::from("Erro ao ler o arquivo pessoas.json"))?;

    // Debug: Verificar o conteúdo lido do arquivo
    println!("Conteúdo lido do arquivo: {}", conteudos);

    let mut pessoas: Vec<Pessoa> = serde_json::from_str(&conteudos)
        .map_err(|_| String::from("Erro ao deserializar as pessoas")) // Tratamento de erro
        .unwrap_or_else(|_| vec![]); // Retorna um vetor vazio em caso de erro

    if pessoas.iter().any(|p| p.cpf == cpf_pessoa) {
        return Err(String::from("Pessoa com esse CPF já cadastrada"));
    }

    let pessoa = Pessoa {
        id: Uuid::new_v4(),
        nome: nome_pessoa,
        cpf: cpf_pessoa,
    };

    pessoas.push(pessoa.clone());
    let pessoas_json = serde_json::to_string(&pessoas).map_err(|_| String::from("Erro ao serializar as pessoas"))?;

    // Limpar o arquivo antes de escrever
    file.set_len(0).map_err(|_| String::from("Erro ao limpar o arquivo antes de escrever"))?;
    file.seek(SeekFrom::Start(0)).map_err(|_| String::from("Erro ao reposicionar o cursor no arquivo"))?;//reposicionar o cursor
    file.write_all(pessoas_json.as_bytes()).map_err(|_| String::from("Erro ao escrever no arquivo pessoas.json"))?;
    file.flush().map_err(|_| String::from("Erro ao garantir que os dados sejam gravados"))?; // Garante que os dados sejam escritos

    Ok(pessoa)
}

pub fn buscar_pessoa(cpf_pessoa: String) -> Result<Pessoa, String> {
   // let dir = "pessoas.json";

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false) // Não cria o arquivo se não existir
        .open("pessoas.json")
        .map_err(|_| String::from("Erro ao abrir o arquivo pessoas.json"))?;

    let mut conteudos = String::new();
    file.read_to_string(&mut conteudos)
        .map_err(|_| String::from("Erro ao ler o arquivo pessoas.json"))?;

    // Remover espaços em branco desnecessários
    let conteudos = conteudos.trim();

    // Verificar o conteúdo lido do arquivo
    println!("Conteúdo lido do arquivo: {}", conteudos);

    let pessoas: Vec<Pessoa> = serde_json::from_str(conteudos)
        .map_err(|_| String::from("Erro ao deserializar as pessoas"))?;

    for pessoa in pessoas {
        if pessoa.cpf == cpf_pessoa {
            return Ok(pessoa); // Retorna a pessoa encontrada
        }
    }

    Err(String::from("Pessoa não encontrada"))
}
