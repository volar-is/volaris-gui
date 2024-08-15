use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use tauri::{command, AppHandle};
use tauri_plugin_dialog::DialogExt;

#[command]
fn create_key_file(path: PathBuf, name: String) -> Result<String, String> {
    let mut name = name;
    let temp_path = PathBuf::from(&name);

    if !name.contains(".key") {
        if temp_path.extension().is_none() {
            name.push_str(".key");
        }
    }
    let mut key_file_path = path;
    key_file_path.push(name);

    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..32).map(|_| rng.random()).collect();

    let mut file = File::create(key_file_path).map_err(|e| e.to_string())?;
    file.write_all(&key).map_err(|e| e.to_string())?;

    Ok(String::from("All good."))
}

#[command]
async fn encrypt_file_with_key(
    app: AppHandle,
    input: String,
    output: String,
    keyfile: String,
) -> Result<String, String> {
    let file_exists = std::path::Path::new(&output).exists();

    if file_exists {
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

    let status = Command::new("bash")
        .args([
            "-c",
            &format!("echo y | volaris-cli -ek {} {} {}", keyfile, input, output),
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok("Encryption successful".to_string())
    } else {
        Err("Failed to encrypt file".to_string())
    }
}

#[command]
async fn decrypt_file_with_key(
    app: AppHandle,
    input: String,
    output: String,
    keyfile: String,
) -> Result<String, String> {
    let file_exists = std::path::Path::new(&output).exists();

    if file_exists {
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

    let status = Command::new("bash")
        .args([
            "-c",
            &format!("echo y | volaris-cli -dk {} {} {}", keyfile, input, output),
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok("Decryption successful".to_string())
    } else {
        Err("Failed to decrypt file".to_string())
    }
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
