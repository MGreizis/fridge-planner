use scdb::Store;
use serde::{Deserialize, Serialize};

// Define an Appliance struct to represent the data structure
#[derive(Serialize, Deserialize, Debug)]
struct Appliance {
    name: String,
    checked: bool,
}

#[tauri::command]
pub fn save_appliances(_app: tauri::AppHandle, appliances: String) {
    // Deserialize the JSON data into a vector of Appliance
    let appliances: Vec<Appliance> =
        serde_json::from_str(&appliances).unwrap_or_else(|_| Vec::new());

    // Get the names of selected appliances
    let selected_appliances: Vec<String> = appliances
        .iter()
        .filter(|appliance| appliance.checked)
        .map(|appliance| appliance.name.clone())
        .collect();

    println!("Saving appliances: {:?}", selected_appliances);

    // Set up and configure your storage (create a store instance)
    let mut store = Store::new(
        "scdb_dumps/appliances",
        Some(1000),
        Some(1),
        Some(10),
        Some(1800),
        true,
    )
    .expect("create store");

    // Use the store to save the data
    for appliance in appliances {
        let key = &appliance.name;
        let value = serde_json::to_string(&appliance).unwrap();
        store
            .set(key.as_bytes(), value.as_bytes(), None)
            .expect("failed to set");
    }
}
