mod source;

#[tauri::command]
async fn handle_download_file(
    source: &str,
    url: &str,
    batch_size: usize,
) -> Result<Vec<String>, String> {
    source::download_file(source, url, batch_size).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![handle_download_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
