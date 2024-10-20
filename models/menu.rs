use std::io;

use crate::models::{
    emprestimo::{devolver, emprestar},
    livro::{buscar_livro, cadastrar_livro, listar_livros_disponiveis},
    pessoa::{buscar_pessoa, cadastrar_pessoa},
};

pub fn menu() {
    println!("BEM-VINDO AO SIBLIB");
    println!("O QUE DESEJA FAZER?");
    println!("1 - Cadastro de livro");
    println!("2 - Empréstimo");
    println!("3 - Devolução");
    println!("4 - Cadastro de pessoas");
    println!("5 - Listar livros disponíveis");
    println!("0 - Sair");

    let mut op = String::new();
    let mut nome = String::new();
    let mut nome_autor = String::new();
    let mut isbn = String::new();
    let mut cpf = String::new();

    while op != "0" {
        op.clear();
        println!("Escolha uma opção:");
        io::stdin()
            .read_line(&mut op)
            .expect("Falha ao ler a linha");
        isbn.clear();
        nome.clear();
        nome_autor.clear();

        match op.trim() {
            "1" => {
                println!("ISBN: ");
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Falha ao ler a linha");
                println!("Nome do livro: ");
                io::stdin()
                    .read_line(&mut nome)
                    .expect("Falha ao ler a linha");
                println!("Nome do Autor: ");
                io::stdin()
                    .read_line(&mut nome_autor)
                    .expect("Falha ao ler a linha");

                match cadastrar_livro(
                    isbn.trim().to_string(),
                    nome.trim().to_string(),
                    nome_autor.trim().to_string(),
                ) {
                    Ok(livro) => println!("Livro cadastrado com sucesso: {:?}", livro),
                    Err(e) => println!("Erro ao cadastrar o livro: {}", e),
                }
            }
            "2" => {
                cpf.clear();
                isbn.clear();

                println!("CPF: ");
                io::stdin()
                    .read_line(&mut cpf)
                    .expect("Falha ao ler a linha");

                match buscar_pessoa(cpf.trim().to_string()) {
                    Ok(pessoa) => {
                        println!("ISBN do livro: ");
                        io::stdin()
                            .read_line(&mut isbn)
                            .expect("Falha ao ler a linha");

                        match buscar_livro(isbn.trim().to_string()) {
                            Ok(livro) => match emprestar(pessoa, livro) {
                                Ok(_) => println!("Empréstimo realizado com sucesso."),
                                Err(e) => println!("Erro ao realizar o empréstimo: {}", e),
                            },
                            Err(e) => println!("Erro ao buscar o livro: {}", e),
                        }
                    }
                    Err(e) => println!("Erro ao buscar a pessoa: {}", e),
                }
            }
            "3" => {
                cpf.clear();
                isbn.clear();

                println!("CPF: ");
                io::stdin()
                    .read_line(&mut cpf)
                    .expect("Falha ao ler a linha");

                match buscar_pessoa(cpf.trim().to_string()) {
                    Ok(pessoa) => {
                        println!("ISBN do livro: ");
                        io::stdin()
                            .read_line(&mut isbn)
                            .expect("Falha ao ler a linha");
                        
                        match devolver(pessoa,isbn.trim().to_string()){
                            Ok(_) => println!("Devolução realizada com sucesso."),
                            Err(e) => println!("Erro ao realizar a devolução: {}", e),
                        }
                    }
                    Err(e) => println!("Erro ao buscar a pessoa: {}", e),
                }
            }
            "4" => {
                nome.clear();
                cpf.clear();

                println!("Cadastro de pessoas");
                println!("Nome: ");
                io::stdin()
                    .read_line(&mut nome)
                    .expect("Falha ao ler a linha");
                println!("CPF: ");
                io::stdin()
                    .read_line(&mut cpf)
                    .expect("Falha ao ler a linha");

                match cadastrar_pessoa(nome.trim().to_string(), cpf.trim().to_string()) {
                    Ok(pessoa) => println!("Pessoa cadastrada com sucesso: {:?}", pessoa),
                    Err(e) => println!("Erro ao cadastrar a pessoa: {}", e),
                }
            }
            "5" => {
                println!("Lista de livros disponíveis");
                listar_livros_disponiveis();
            }
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Opção inválida, tente novamente.");
            }
        }
    }
}
