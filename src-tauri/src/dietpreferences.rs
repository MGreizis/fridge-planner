use scdb::Store;
use serde::{Deserialize, Serialize};

// Define a DietPreference struct to represent the data structure
#[derive(Serialize, Deserialize, Debug)]
struct DietPreference {
    name: String,
    selected: bool,
}

#[tauri::command]
pub fn save_dietary_preferences(_app: tauri::AppHandle, preferences: String) {
    let mut dietary_preferences: Vec<DietPreference> =
        serde_json::from_str(&preferences).unwrap_or_else(|_| Vec::new());

    // Get the names of selected preferences
    let selected_preferences: Vec<String> = dietary_preferences
        .iter()
        .filter(|preference| preference.selected)
        .map(|preference| preference.name.clone())
        .collect();

    // Now you have a vector of DietPreference objects that you can work with
    println!("Saving dietary preferences: {:?}", selected_preferences);

    // Set up and configure your storage (create a store instance)
    let mut store = Store::new(
        "scdb_dumps",
        Some(1000),
        Some(1),
        Some(10),
        Some(1800),
        true,
    )
    .expect("create store");

    // Use the store to save the data
    for preference in dietary_preferences.iter() {
      let key = &preference.name;
      let value = serde_json::to_string(preference).unwrap();
      let result = store
          .set(key.as_bytes(), value.as_bytes(), None);

      match result {
          Ok(()) => println!("Saved dietary preference: {}", key),
          Err(err) => eprintln!("Failed to save preference {}: {:?}", key, err),
      }
  }
}
