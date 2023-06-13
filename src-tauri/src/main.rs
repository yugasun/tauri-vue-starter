// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api, CustomMenuItem, Manager, Menu, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_positioner;

mod cmds;

fn main() {
    // add sample menu items
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let github = CustomMenuItem::new("github".to_string(), "View on Github");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github);
    let tray: SystemTray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    let mut builder = tauri::Builder::default();

    // init tauri plugins
    builder = builder.plugin(tauri_plugin_positioner::init());

    // add commands
    builder = builder.invoke_handler(tauri::generate_handler![cmds::hello]);

    // setup the window
    builder = builder.setup(|app| {
        let main_window = app.get_window("main").unwrap();
        main_window.show().unwrap();
        main_window.set_focus().unwrap();
        Ok(())
    });

    // build the menu
    builder = builder
        .menu(Menu::os_default(&context.package_info().name))
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    // quit the app
                    "quit" => {
                        std::process::exit(0);
                    }
                    // open github link
                    "github" => {
                        api::shell::open(
                            &app.get_window("main").unwrap().shell_scope(),
                            "https://github.com/yugasun/tauri-vue-starter".to_string(),
                            None,
                        )
                        .unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        });

    builder
        .run(context)
        .expect("error while running tauri application");
}
