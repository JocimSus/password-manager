// 1. Create simple UI for command line
// 2. Create sqlite database to store logins && passwords
// 3. Safely store passwords using bcrypt

extern crate bcrypt;
use sqlite::State;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use toml::Table;

const DB_PATH: &str = "./passwords.sqlite";
const CONFIG_PATH: &str = "./config.toml";

fn main() {
    // Init
    if !Path::new(DB_PATH).exists() {
        init_db();
    } else if !Path::new(CONFIG_PATH).exists() {
        init_config()
    }

    loop {
        let display_rows = read_config() as i64;
        println!("||PASSWORD MANAGER||");
        println!("1. Show Passwords");
        println!("2. Insert Password");
        println!("3. Edit Config");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: Unable to get input");

        println!("");

        if input.trim() == "1" {
            let connection =
                sqlite::Connection::open(DB_PATH).expect("Error: Failed to connect to database");
            let query = "
                SELECT * FROM logins LIMIT ?
            ";
            let mut statement = connection
                .prepare(query)
                .expect("Error: Failed to prepare connection to database");
            statement.bind((1, display_rows)).unwrap();
            while let Ok(State::Row) = statement.next() {
                println!("Row");
                println!("name = {}", statement.read::<String, _>("site").unwrap());
                println!(
                    "password = {}",
                    statement.read::<String, _>("password").unwrap()
                );
            }
        } else {
            break;
        }
    }
}

fn init_db() {
    let connection = sqlite::Connection::open(DB_PATH).unwrap();

    let query = "
        CREATE TABLE logins (site TEXT, user TEXT, password TEXT);
    ";
    connection.execute(query).unwrap();
}

fn init_config() {}

fn read_config() -> i64 {
    let mut config_file = File::open(CONFIG_PATH).expect("Error: Cannot open config file");
    let mut contents = String::new();
    config_file
        .read_to_string(&mut contents)
        .expect("Error: Failed to read config file");

    let toml_table = contents
        .parse::<Table>()
        .expect("Error: Failed to parse config toml");

    let mut display_rows: i64 = 5;
    if let Some(config) = toml_table.get("config") {
        let config_table = config.as_table();
        if let Some(table) = config_table {
            if table.clone().get("display_rows").is_some() {
                display_rows = table["display_rows"].as_integer().unwrap();
            }
        }
    }

    return display_rows;
}
