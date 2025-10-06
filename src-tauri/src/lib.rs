// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod router;
mod widget_loader; // declares widget_loader.rs as a module
mod widget_api;    // declare other modules
mod utils;
use tokio::signal;
use tokio;
#[cfg_attr(mobile, tauri::mobile_entry_point)]


#[tokio::main]
pub async fn run() {
    // Load widgets
    let widget_path = utils::get_widgets_path();
    utils::verify_config(&widget_path);

    tokio::spawn(widget_api::widget_server(widget_path));

    // Now i need http server that servers list of widget and widget html of choice
    // GET /           Json list of widgets
    // GET /WidgetName Html of specific widget
    
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    signal::ctrl_c().await.unwrap()
}
