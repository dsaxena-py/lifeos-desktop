// Aeon desktop is a thin native shell — the single window points straight at
// the deployed web app (see tauri.conf.json's app.windows[0].url) rather
// than bundling any local frontend. No custom commands are needed since the
// page never calls back into Rust.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
