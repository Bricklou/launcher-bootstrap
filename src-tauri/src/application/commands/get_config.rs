#[tauri::command]
pub fn get_config(config_name: String) -> bool {
    println!("get_config: {}", config_name);
    true
}
