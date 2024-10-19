use uuid::Uuid;

mod models;

use models::livro::Livro;
use models::pessoa::cadastrar_pessoa;
use crate::models::livro::cadastrar_livro;
use crate::models::livro::Listar;



fn main() {
    let livros = vec![Livro {
        id: Uuid::new_v4(),
        isbn: String::from("978-1-23456-789-0"),
        nome: String::from("Rust Book"),
        nomeautor: String::from("Autor Desconhecido"),
    }];

    match cadastrar_pessoa(String::from("Fulano"), String::from("12345678900"), livros) {
        Ok(pessoa) => println!("Pessoa cadastrada com sucesso: {:?}", pessoa),
        Err(err) => eprintln!("Erro ao cadastrar pessoa: {}", err),
    }
   let livro = Livro::new(
        String::from("978-1-23456-789-0"),
        String::from("Rust Book"),
        String::from("Autor Desconhecido"),
    );
    let livro = livro;

    // Exibir informações do livro
    println!("{}", livro.listar_struct());

    // Cadastrar um livro (exemplo)
    match cadastrar_livro(
        String::from("978-1-23456-789-0"),
        String::from("Rust Book"),
        String::from("Autor Desconhecido"),
    ) {
        Ok(livro) => println!("Livro cadastrado: {:?}", livro),
        Err(e) => println!("Erro: {}", e),
    }
}
