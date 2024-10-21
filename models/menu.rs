use std::io;

use crate::models::{
    emprestimo::{devolver, emprestar, listar_emprestados},
    livro::{buscar_livro, cadastrar_livro, listar_livros_disponiveis,listarlivroporautor,listarlivroporano},
    pessoa::{buscar_pessoa, cadastrar_pessoa},
    pagamento::{listarpagamentos,cadastrar_pagamento},
    multa::{cadastrarmulta,listarmultas},
};

fn perfumaria_menu(){
    println!("BEM-VINDO AO SIBLIB");
    println!("O QUE DESEJA FAZER?");
    println!("1 - Cadastro de livro");
    println!("2 - Empréstimo");
    println!("3 - Devolução");
    println!("4 - Cadastro de pessoas");
    println!("5 - Listar livros disponíveis");
    println!("6 - Listar livros emprestados");
    println!("7 - Listar livros por autor");	
    println!("8 - Listar livros por ano");
    println!("9 - Realizar pagamento");
    println!("10 - Listar pagamentos");
    println!("11 - Listar multas");
    println!("0 - Sair");
}

pub fn menu() {

    let mut op = String::new();
    let mut nome = String::new();
    let mut nome_autor = String::new();
    let mut isbn = String::new();
    let mut cpf = String::new();
    let mut ano = String::new();
    let valor_: f64;

    while op != "0" {
        perfumaria_menu();
        op.clear();
        println!("Escolha uma opção:");
        io::stdin()
            .read_line(&mut op)
            .expect("Falha ao ler a linha");
        isbn.clear();
        nome.clear();
        nome_autor.clear();
        ano.clear();

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
                println!("Ano: ");
                io::stdin()
                    .read_line(&mut ano)
                    .expect("Falha ao ler a linha");

                match cadastrar_livro(
                    isbn.trim().to_string(),
                    nome.trim().to_string(),
                    nome_autor.trim().to_string(),
                    ano.trim().to_string(),
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
                listar_livros_disponiveis();
            }
            "6" => {
                println!("Livros emprestados: ");
                listar_emprestados();
            }
            "7" => {
                nome_autor.clear();
                println!("Nome do autor:");
                io::stdin()
                    .read_line(&mut nome_autor)
                    .expect("Falha ao ler a linha");
                listarlivroporautor(nome_autor.trim().to_string());
            }
            "8" => {
                ano.clear();
                println!("Ano dos livros:");
                io::stdin()
                    .read_line(&mut ano)
                    .expect("Falha ao ler a linha");
                listarlivroporano(ano.trim().to_string());
            }
            "9" => {
                cpf.clear();
                println!("CPF: ");
                io::stdin()
                    .read_line(&mut cpf)
                    .expect("Falha ao ler a linha");
                
                println!("Valor: ");
               
                let mut valor_str = String::new();
                io::stdin()
                    .read_line(&mut valor_str)
                    .expect("Falha ao ler o valor do pagamento");
            
                // Converter a string para f64
                let valor: f64 = match valor_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Valor inválido. Por favor, digite um número.");
                        continue;
                    }
                };

                match buscar_pessoa(cpf.trim().to_string()) {
                    Ok(pessoa) => {
                        match cadastrar_pagamento(pessoa, valor) {
                            Ok(_) => println!("Pagamento cadastrado com sucesso."),
                            Err(e) => println!("Erro ao cadastrar o pagamento: {}", e),
                        }
                    }
                    Err(e) => println!("Erro ao buscar a pessoa: {}", e),
                }
        }
        "10" => {
                listarpagamentos();
            }
        "11" => {
                println!("Multas:");
                listarmultas();
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
