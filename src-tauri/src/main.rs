use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use tauri::{command, AppHandle};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use volaris_crypto::header::{
    HashingAlgorithm, HeaderType, HeaderVersion, ARGON2ID_LATEST, BLAKE3BALLOON_LATEST,
};
use volaris_crypto::key::{argon2id_hash, balloon_hash, generate_passphrase};
use volaris_crypto::primitives::{gen_salt, Algorithm, Mode};
use volaris_crypto::protected::Protected;
use volaris_tools::decrypt::{execute as d_execute, Request as d_request};
use volaris_tools::encrypt::{execute as e_execute, Request as e_request};

#[command]
async fn create_key_file(
    app: AppHandle,
    path: PathBuf,
    name: String,
    hash: String,
    header: String,
) -> Result<String, String> {
    let mut name = name;
    if !name.ends_with(".key") {
        name.push_str(".key");
    }

    let mut key_file_path = path;
    key_file_path.push(name);
    check_file_exists(&app, &key_file_path)
        .await
        .map_err(|e| e.to_string())?;
    let now = std::time::Instant::now();
    let salt = gen_salt();
    let secret_data = generate_passphrase(&32).as_bytes().to_vec();
    let raw_key = Protected::new(secret_data);
    let elapsed = now.elapsed();
    eprintln!("Raw Key Elapsed: {:.2?}", elapsed);

    let header_version_enum = match header.to_lowercase().as_str() {
        "v3" => HeaderVersion::V3,
        "v4" => HeaderVersion::V4,
        "v5" => HeaderVersion::V5,
        _ => return Err(format!("Unsupported header version: {}", header).into()),
    };

    let key = match hash.as_str() {
        "Blake3Balloon" => {
            balloon_hash(raw_key, &salt, &header_version_enum).map_err(|e| e.to_string())?
        }
        "Argon2ID" => {
            argon2id_hash(raw_key, &salt, &header_version_enum).map_err(|e| e.to_string())?
        }
        _ => return Err(format!("Unsupported hash algorithm: {}", hash).into()),
    };

    fs::write(key_file_path, key.deref()).map_err(|e| e.to_string())?;
    eprintln!("Key Elapsed: {:.2?}", now.elapsed());

    Ok("Key file created successfully.".to_string())
}

async fn check_file_exists(app: &AppHandle, path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        let answer = app
            .dialog()
            .message("The file already exists. Do you want to overwrite it?")
            .title("Confirm Overwrite")
            .buttons(MessageDialogButtons::OkCancel)
            .blocking_show();

        if !answer {
            return Err("Operation canceled by the user.".to_string());
        }
    }
    Ok(())
}

#[command]
// Updated to support multiple hashing and encryption methods
async fn encrypt_file_with_key(
    app: AppHandle,
    input: String,
    output: String,
    keyfile: String,
    hash: String,
    ealgorithm: String,
    headerver: String,
) -> Result<String, String> {
    let input_path = PathBuf::from(input);
    let output_path = PathBuf::from(output);
    let keyfile_path = PathBuf::from(keyfile);

    check_file_exists(&app, &output_path)
        .await
        .map_err(|e| e.to_string())?;
    println!("Running Encrypt");

    let mut keyfile = File::open(keyfile_path).map_err(|e| e.to_string())?;
    let mut key_data = Vec::new();
    keyfile
        .read_to_end(&mut key_data)
        .map_err(|e| e.to_string())?;

    if key_data.len() != 32 {
        return Err("Invalid key size. The key must be 32 bytes long.".to_string());
    }

    let mut input_file = File::open(input_path).map_err(|e| e.to_string())?;
    let mut input_data = Vec::new();
    input_file
        .read_to_end(&mut input_data)
        .map_err(|e| e.to_string())?;

    let input_cursor = std::io::Cursor::new(input_data);
    let output_file = File::create(&output_path).map_err(|e| e.to_string())?;

    let hash = match hash.as_str() {
        "Blake3Balloon" => HashingAlgorithm::Blake3Balloon(BLAKE3BALLOON_LATEST),
        "Argon2ID" => HashingAlgorithm::Argon2id(ARGON2ID_LATEST),
        _ => return Err("Unsupported hashing algorithm.".to_string()),
    };

    let ealgorithm = match ealgorithm.as_str() {
        "AES-256-GCM" => Algorithm::Aes256Gcm,
        "XChaCha20Poly1305" => Algorithm::XChaCha20Poly1305,
        _ => return Err("Unsupported encryption algorithm.".to_string()),
    };

    let headerver = match headerver.to_lowercase().as_str() {
        "v3" => HeaderVersion::V3,
        "v4" => HeaderVersion::V4,
        "v5" => HeaderVersion::V5,
        _ => return Err(format!("Unsupported header version: {}", headerver).into()),
    };

    let request = e_request {
        reader: &RefCell::new(input_cursor),
        writer: &RefCell::new(output_file),
        header_writer: None,
        raw_key: Protected::new(key_data),
        hashing_algorithm: hash,
        header_type: HeaderType {
            version: headerver,
            algorithm: ealgorithm,
            mode: Mode::StreamMode,
        },
    };

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

    check_file_exists(&app, &output_path)
        .await
        .map_err(|e| e.to_string())?;

    let mut keyfile = File::open(keyfile_path).map_err(|e| e.to_string())?;
    let mut key_data = Vec::new();
    keyfile
        .read_to_end(&mut key_data)
        .map_err(|e| e.to_string())?;

    if key_data.len() != 32 {
        return Err("Invalid key size. The key must be 32 bytes long.".to_string());
    }

    let input_file = File::open(input_path).map_err(|e| e.to_string())?;
    let output_file = File::create(output_path).map_err(|e| e.to_string())?;
    let reader = RefCell::new(input_file);
    let writer = RefCell::new(output_file);

    let now = std::time::Instant::now();
    let protected_key = Protected::new(key_data);

    d_execute(d_request {
        header_reader: None,
        reader: &reader,
        writer: &writer,
        raw_key: protected_key,
        on_decrypted_header: None,
    })
    .map_err(|e| e.to_string())?;

    eprintln!("Elapsed: {:.2?}", now.elapsed());
    Ok("Decryption successful".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_key_file,
            encrypt_file_with_key,
            decrypt_file_with_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
