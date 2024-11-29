use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

mod db;
mod controllers;
mod routes;
mod models;
mod utils;

use db::set_database;
use controllers::{handle_post_request, handle_get_request, handle_get_all_request, handle_put_request, handle_delete_request};
use routes::get_route_handler;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

fn main() {
    if let Err(e) = set_database() {
        println!("Erro ao configurar o banco de dados: {}", e);
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Servidor iniciado na porta 8080");

    // Escutar as requisições
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Erro na conexão: {}", e),
        }
    }
}

// Função que escolhe o manipulador com base na requisição
pub fn get_route_handler(request: &str) -> (String, String) {
    match &*request {
        r if r.starts_with("POST /users") => handle_post_request(r),
        r if r.starts_with("GET /users/") => handle_get_request(r),
        r if r.starts_with("GET /users") => handle_get_all_request(),
        r if r.starts_with("PUT /users/") => handle_put_request(r),
        r if r.starts_with("DELETE /users/") => handle_delete_request(r),
        _ => ("HTTP/1.1 404 NOT FOUND\r\n".to_string(), "404 Not Found".to_string())
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = get_route_handler(&request);

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Erro ao ler requisição: {}", e);
        }
    }
}
