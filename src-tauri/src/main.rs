// cm2labs Fluyo - Core v1.2.2
// Nomenclatura: fluyo-by-cm2labs.exe
// Bóveda Local: Portabilidad Estricta
// Context7 Documentation Sync: Verified with Tauri v1.5 official docs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Garantizar la creación de la carpeta storage al inicio usando ruta canónica
            let exe_path = std::env::current_exe().expect("Failed to get exe path");
            let base_dir = exe_path.parent().unwrap();
            let storage_dir = base_dir.join("storage");
            
            if !storage_dir.exists() {
                fs::create_dir_all(&storage_dir).expect("Failed to create storage directory");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
