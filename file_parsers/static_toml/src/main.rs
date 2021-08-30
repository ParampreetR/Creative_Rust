extern crate toml;

use serde_derive::Deserialize;

// Create structures for getting toml file fields
#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[derive(Deserialize)]
struct Redis {
    host: String,
}

#[derive(Deserialize)]
struct Sqlite {
    db_file: String,
}

#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}




fn main() {
    let config_values: Config = {
        let config_file_path = std::env::args().nth(1).unwrap();
        let config_file_content = std::fs::read_to_string(&config_file_path).unwrap();  // Read from file to string
        toml::from_str(&config_file_content).unwrap()   // deserialize from raw string. Variable must have Deserialized trait implemented.
    };

    println!("{}", config_values.input.xml_file);
}
