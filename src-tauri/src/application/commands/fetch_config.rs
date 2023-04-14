#[tauri::command]
pub fn get_or_fetch_config(url: String) -> Option<bool> {
    println!("get_or_fetch_config: {}", url);
    None
}
