#[tauri::command]
pub fn save_preferences(app: tauri::AppHandle, appliances: String) {
    let appliances: Vec<String> = serde_json::from_str(&appliances)
        .unwrap_or_else(|_| Vec::new());

    println!("Saving appliances: {:?}", appliances);
}