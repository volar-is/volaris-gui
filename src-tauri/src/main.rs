use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::cell::RefCell;
use tauri::{command, AppHandle};
use tauri_plugin_dialog::DialogExt;
use volaris_tools::decrypt::{Request as d_request, execute as d_execute};
use volaris_tools::encrypt::{Request as e_request, execute as e_execute};
use volaris_crypto::protected::Protected; 
use volaris_crypto::header::{HeaderVersion, HashingAlgorithm, HeaderType};
use volaris_crypto::primitives::{Mode, Algorithm};

#[command]
fn create_key_file(path: PathBuf, name: String) -> Result<String, String> {
    let mut name = name;
    if !name.ends_with(".key") {
        name.push_str(".key");
    }

    let mut key_file_path = path;
    key_file_path.push(name);

    // Generate a 32-byte key
    let raw_key: [u8; 32] = rand::random();

    // Write the key to a file
    let mut file = File::create(key_file_path).map_err(|e| e.to_string())?;
    file.write_all(&raw_key).map_err(|e| e.to_string())?;

    Ok("Key file created successfully.".to_string())
}

fn check_file_exists(app: &AppHandle, path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        let answer = app
            .dialog()
            .message("The file already exists. Do you want to overwrite it?")
            .title("Confirm Overwrite")
            .ok_button_label("Yes")
            .cancel_button_label("No")
            .blocking_show();

        if !answer {
            return Err("Operation canceled by the user.".to_string());
        }
    }
    Ok(())
}


#[command]
async fn encrypt_file_with_key(
    app: AppHandle,
    input: String,
    output: String,
    keyfile: String,
) -> Result<String, String> {
    let input_path = PathBuf::from(input);
    let output_path = PathBuf::from(output);
    let keyfile_path = PathBuf::from(keyfile);

    check_file_exists(&app, &output_path)?;
    print!("Running Encrypt");

    // Read key from keyfile
    let mut keyfile = File::open(keyfile_path).map_err(|e| e.to_string())?;
    let mut key_data = Vec::new();
    keyfile.read_to_end(&mut key_data).map_err(|e| e.to_string())?;

    if key_data.len() != 32 {
        return Err("Invalid key size. The key must be 32 bytes long.".to_string());
    }

    // Read input file
    let mut input_file = File::open(input_path).map_err(|e| e.to_string())?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).map_err(|e| e.to_string())?;

    // Create a buffer as a cursor for encryption
    let input_cursor = std::io::Cursor::new(input_data);

    // Set up the necessary I/O using RefCell for mutable borrows
    let mut output_file = File::create(&output_path).map_err(|e| e.to_string())?;

    // Create a request for encryption
    let request = e_request {
        reader: &RefCell::new(input_cursor),
        writer: &RefCell::new(output_file),
        header_writer: None, // Add appropriate value if needed
        raw_key: Protected::new(key_data),
        hashing_algorithm: HashingAlgorithm::Blake3Balloon(4),
        header_type: HeaderType {
            version: HeaderVersion::V5,
            algorithm: Algorithm::XChaCha20Poly1305,
            mode: Mode::StreamMode,
        },
    };

    // Execute the encryption
    e_execute(request).map_err(|e| e.to_string())?;
    print!("Encrypt Finished");
    Ok("Encryption successful".to_string())
}

#[command]
async fn decrypt_file_with_key(
    app: AppHandle,
    input: String,
    output: String,
    keyfile: String,
) -> Result<String, String> {
    let input_path = PathBuf::from(input);
    let output_path = PathBuf::from(output);
    let keyfile_path = PathBuf::from(keyfile);

    check_file_exists(&app, &output_path)?;
    print!("Running Decrypt");

    // Read key from keyfile
    let mut keyfile = File::open(keyfile_path).map_err(|e| e.to_string())?;
    let mut key_data = Vec::new();
    keyfile.read_to_end(&mut key_data).map_err(|e| e.to_string())?;

    if key_data.len() != 32 {
        return Err("Invalid key size. The key must be 32 bytes long.".to_string());
    }

    // Set up the necessary I/O using RefCell for mutable borrows
    let input_file = File::open(input_path).map_err(|e| e.to_string())?;
    let output_file = File::create(output_path).map_err(|e| e.to_string())?;
    let reader = RefCell::new(input_file);
    let writer = RefCell::new(output_file);

    // Create a request for decryption
    let req = d_request {
        header_reader: None, // Add appropriate value if needed
        reader: &reader,
        writer: &writer,
        raw_key: Protected::new(key_data),
        on_decrypted_header: None, // Add appropriate value if needed
    };

    // Execute the decryption
    d_execute(req).map_err(|e| e.to_string())?;

    print!("Decrypt finished.");
    Ok("Decryption successful".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_key_file,
            encrypt_file_with_key,
            decrypt_file_with_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

