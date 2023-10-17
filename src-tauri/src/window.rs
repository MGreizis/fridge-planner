use tauri::WindowBuilder;

#[tauri::command]
pub async fn create_window(app: tauri::AppHandle) {
  let _window = tauri::WindowBuilder::new(&app, "label", tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()))
    .build()
    .unwrap();
}