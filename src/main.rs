use reqwest::Client;
use std::collections::HashMap;
use std::io::Write;

async fn get_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.get(url);

    if let Some(p) = params {
        request = request.query(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

async fn post_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.post(url);

    if let Some(p) = params {
        request = request.form(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

// I hate my life
fn input(prompt: &str) -> String {
    let mut cmd: String = Default::default();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut cmd).unwrap();
    cmd = cmd.trim().to_string();

    return cmd;
}

macro_rules! hashmap {
    ( $( $x:expr => $x1:expr ),* ) => {
        {
            let mut _temp_hashmap = HashMap::new();
            $(
                _temp_hashmap.insert($x, $x1);
            )*
            _temp_hashmap
        }
    };

    () => {
        HashMap::new()
    }
}

#[tokio::main]
async fn main() {
    let main_url = "http://bov.puppet57.xyz/php/";

    let account_exists = input("Have you made an account before? [Y/n]: ");

    loop {
        if account_exists == "n" || account_exists == "N" {
            println!("Please note that if your username isnt your valorian name your account will be deleted!");
            let username = input("Please input your username: ");
            let password = input("Please input your password: ");

            let register_successful = post_request(
                format!("{}register-account.php", main_url),
                Some( hashmap! {
                    "user".to_string() => username,
                    "pass".to_string() => password
                })
            ).await;

            if register_successful == "user_exists" {
                println!("User already exists!\n");
            } else if register_successful == "user_made" {
                println!("Account has been created!");
            } else {
                println!("There was an error: {}", register_successful);
            }
        } else if account_exists == "y" || account_exists == "Y" {
            let username = input("Please input your username: ");
            let password = input("Please input your password: ");

            let login_successful = post_request(
                format!("{}login.php", main_url),
                Some( hashmap! {
                    "user".to_string() => username,
                    "pass".to_string() => password
                })
            ).await;

            if login_successful == "user_doesnt_exist" {
                println!("That user doesn't exist!");
            } else if login_successful == "login_success" {
                println!("You've been logged in!\n");
                break;
            } else if login_successful == "incorrect_password" {
                println!("Your password is incorrect!");
            } else {
                println!("There was an error: {}", login_successful);
            }
        } else {
            println!("Invalid input");
        }
    }

    println!("What would you like to do?");
    println!("Type help for a list of commands!");

    loop {
        let cmd = input("> ");

        if cmd == "pay" {
            println!("Paying people will be added later!");
        } else if cmd == "help" {
            println!("1: pay (Pays a specific user a certain amount of money)");
            println!("2: exit");
        } else if cmd == "exit" {
            break;
        } else {
            println!("{} is an invalid command!", cmd);
        }
    }
}