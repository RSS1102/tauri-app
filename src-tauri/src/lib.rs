// use tauri::{menu::{MenuBuilder, SubmenuBuilder}, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        // .setup(|app| {
        //     let handle = app.handle();
        //     let file_menu = SubmenuBuilder::new(handle, "file")
        //         .text("new_file", "new file")
        //         .text("open_file", "open file")
        //         .text("save", "save")
        //         .text("save_as", "save as")
        //         .build()?;

        //     let menu = MenuBuilder::new(app).item(&file_menu).build()?;
        //     let _ = app.set_menu(menu);

        //     app.on_menu_event(|app_handle, event| match event.id().0.as_str() {
        //         "open_file" => {
        //             println!("open file");
        //             let _ = app_handle.emit("open_file", "");
        //         }
        //         _ => {}
        //     });

        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
