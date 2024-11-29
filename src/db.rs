use postgres::{Client, NoTls, Error as PostgresError};
use crate::models::User;

const DB_URL: &str = env!("DATABASE_URL");

// Função para configurar o banco de dados
pub fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )"
    )?;
    Ok(())
}

// Função que retorna uma conexão com o banco
pub fn get_db_client() -> Result<Client, PostgresError> {
    Client::connect(DB_URL, NoTls)
}
