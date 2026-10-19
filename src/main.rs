use bs58;
use solana_sdk::signature::{Keypair, Signer};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};

fn main() {
    let secret_key = read_secret_key();

    if secret_key.len() != 64 {
        panic!("Invalid key length: {} (expected 64)", secret_key.len());
    }

    // Create keypair.
    let keypair = Keypair::from_bytes(&secret_key).expect("Invalid secret key");

    // Convert to JSON array.
    let json = serde_json::to_string(&keypair.to_bytes().to_vec()).expect("Failed to serialize");

    // Save to file.
    let output_path = env::current_dir()
        .expect("Failed to determine the current directory")
        .join("id.json");
    let mut file = File::create(&output_path).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write file");

    println!("Keypair saved to {}", output_path.display());

    // Print public key.
    println!("Public key: {}", keypair.pubkey());
}

/// Invite user to type text.
fn prompt(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_owned()
}

// Invite user to type secret.
fn prompt_secret(message: &str) -> String {
    rpassword::prompt_password(message).expect("Failed to read private key securely")
}

// General function to ask user to load his private key.
fn read_secret_key() -> Vec<u8> {
    loop {
        println!("How would you like to provide the private key?");
        println!("1) Enter the Base58 string");
        println!("2) Enter a file path containing the Base58 string");
        println!("3) Enter a JSON byte array (for example: [233, 85, ...])");

        match prompt("Choose 1, 2, or 3: ").as_str() {
            "1" => {
                let key = prompt_secret("Enter the Base58 string (input hidden): ");
                if !key.is_empty() {
                    match bs58::decode(key).into_vec() {
                        Ok(bytes) => return bytes,
                        Err(error) => eprintln!("Failed to decode Base58: {error}\\n"),
                    }
                }
                eprintln!("The Base58 string cannot be empty.\n");
            }
            "2" => {
                let path = prompt("Enter the file path: ");
                match fs::read_to_string(&path) {
                    Ok(contents) if !contents.trim().is_empty() => {
                        match bs58::decode(contents.trim()).into_vec() {
                            Ok(bytes) => return bytes,
                            Err(error) => eprintln!("Failed to decode Base58: {error}\\n"),
                        }
                    }
                    Ok(_) => eprintln!("The file does not contain a Base58 string.\n"),
                    Err(error) => eprintln!("Failed to read '{path}': {error}\n"),
                }
            }
            "3" => {
                let key = prompt_secret("Enter the JSON byte array (input hidden): ");
                match serde_json::from_str::<Vec<u8>>(&key) {
                    Ok(bytes) => return bytes,
                    Err(error) => eprintln!("Invalid JSON byte array: {error}\\n"),
                }
            }
            _ => eprintln!("Invalid choice. Please enter 1, 2, or 3.\n"),
        }
    }
}
