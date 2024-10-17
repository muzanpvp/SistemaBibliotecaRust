use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::File;
use std::io::Write;

struct Livro{
    id: Uuid,    
    isbn: String,
    nome: String,
    nomeautor: String,
    quantidade: u32
}

trait Listar{
    fn listar_struct() -> String;
}

fn cadastrarlivro(nome_str: String, isbn_str: String, nome_str: String, nomeautor_str: String, quantidade_str: u32) -> Livro{
   let dir = "../contexts/livros.json";
   let mut file = File::open(dir).expect("Erro ao abrir arquivo");
   let mut conteudos = String::new();
   file.read_to_string(&mut conteudos).expect("Erro ao ler o arquivo");
   
   let livros: Vec<Livro> = serde_json::from_str(&conteudos).expect("Erro ao desserializar");
   let livro_encontrado = livros.iter().find(|l| l.nome == nome_str);

   //Se não encontrou um livro com o mesmo nome, cria um livro novo
   if(!livro_encontrado){
    let livro = Livro{
        id: Uuid::new_v4();
        nome: nome_str,
        isbn: isbn_str,
        nomeautor: nomeautor_str,
        quantidade: quantidade_str
    };
    let livro_json = serde_json::to_string(&livro).unwrap();
    let mut file = File::create()
    }
}