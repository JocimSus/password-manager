// 1. Create simple UI for command line:
//  1a. Show Passwords after decrypting passwords
//  1b. Search function by site/login
// 2. Create sqlite database to store sites && logins && passwords:
//  2a. Encrypt passwords (bcrypt)
// 3. Create user config file

extern crate bcrypt;
use sqlite::State;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use toml::{Table, Value};

const DB_PATH: &str = "./passwords.sqlite";
const CONFIG_PATH: &str = "./config.toml";
const DEFAULT_DISPLAY_ROWS: i64 = 5;

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

        println!();

        if input.trim() == "1" {
            println!("||Passwords||");
            let connection =
                sqlite::Connection::open(DB_PATH).expect("Error: Failed to connect to database");
            let query = "
                SELECT * FROM logins LIMIT ?;
            ";
            let mut statement = connection
                .prepare(query)
                .expect("Error: Failed to prepare connection to database");
            statement.bind((1, display_rows)).unwrap();
            while let Ok(State::Row) = statement.next() {
                println!(
                    "site = {}, user = {}, password = {}",
                    statement.read::<String, _>("site").unwrap(),
                    statement.read::<String, _>("user").unwrap(),
                    statement.read::<String, _>("password").unwrap()
                );
            }
            println!();
        } else if input.trim() == "2" {
            let connection =
                sqlite::Connection::open(DB_PATH).expect("Error: Failed to connect to database");

            println!("||Insert Password||");
            print!("Site: ");
            io::stdout().flush().unwrap();
            let mut site = String::new();
            io::stdin()
                .read_line(&mut site)
                .expect("Error: Unable to get input");
            let site = site.trim();

            print!("User: ");
            io::stdout().flush().unwrap();
            let mut user = String::new();
            io::stdin()
                .read_line(&mut user)
                .expect("Error: Unable to get input");
            let user = user.trim();

            print!("Password: ");
            io::stdout().flush().unwrap();
            let mut password = String::new();
            io::stdin()
                .read_line(&mut password)
                .expect("Error: Unable to get input");
            let password = password.trim();

            println!();

            let query = format!(
                "
                    INSERT INTO logins VALUES ('{}', '{}', '{}');
                ",
                site, user, password
            );
            connection
                .execute(query)
                .expect("Error: Unable to execute query");
        } else if input.trim() == "3" {
            println!("||Edit Config||");
            print!("Display Rows: ");
            io::stdout().flush().unwrap();
            let mut display_rows = String::new();
            io::stdin()
                .read_line(&mut display_rows)
                .expect("Error: Unable to get input");
            let display_rows: i64 = match display_rows.trim().parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("Error: {}", err);
                    continue;
                }
            };
            write_to_config(display_rows);
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

fn init_config() {
    let mut config_table = Table::new();
    config_table.insert("display_rows".to_owned(), 5.into());

    let mut toml_table = Table::new();
    toml_table.insert("config".to_owned(), toml::Value::Table(config_table));

    let contents = toml::to_string(&Value::Table(toml_table)).unwrap();

    let mut file = File::create(CONFIG_PATH).expect("Error: Failed to create config file");
    file.write_all(contents.as_bytes())
        .expect("Error: Failed to write to config file");
}

fn read_config() -> i64 {
    let mut config_file = File::open(CONFIG_PATH).expect("Error: Cannot open config file");
    let mut contents = String::new();
    config_file
        .read_to_string(&mut contents)
        .expect("Error: Failed to read config file");

    let toml_table = contents
        .parse::<Table>()
        .expect("Error: Failed to parse config toml");

    let mut display_rows: i64 = DEFAULT_DISPLAY_ROWS;
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

fn write_to_config(display_rows: i64) {
    // Code to write to toml
}
