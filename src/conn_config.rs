use serde::Deserialize;

#[derive(Deserialize)]
pub struct Connection {
    pub address: Address,
    pub user: User,
    pub database: Database
}

#[derive(Deserialize)]
pub struct Address {
    pub host: String,
    pub port: u16
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct Database {
    pub namespace: String,
    pub database: String
}

/*
[address]
host = "localhost"
port = 8000

[user]
username = "surreal_test"
password = "surreal_test"

[database]
namespace = "surreal_test"
database = "surreal_test"
 */