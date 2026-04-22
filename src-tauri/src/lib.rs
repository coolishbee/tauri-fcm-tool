mod command;
pub mod fcm;

#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use std::sync::Mutex;
use tauri::Manager;
use tauri_specta::{collect_commands, collect_events};

pub mod modules {
    pub mod logger;
    pub mod types;
}

pub struct AppState {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            // 설정
            command::get_config,
            command::save_config,
            // 인증
            command::is_authenticated,
            command::get_token,
            command::start_oauth,
            command::logout,
            // FCM 발송
            command::send_fcm_message,
            // 템플릿
            command::get_templates,
            command::save_template,
            command::delete_template,
            // 히스토리
            command::get_history,
            command::clear_history,
        ])
        .events(collect_events![]);

    #[cfg(debug_assertions)]
    {
        builder
            .export(Typescript::default(), "../src/lib/bindings.ts")
            .expect("Failed to export typescript bindings");
    }

    let invoke_handler = builder.invoke_handler();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .manage(Mutex::new(AppState {}))
        .setup(move |app| {
            builder.mount_events(app);
            if let Ok(app_data_dir) = app.path().app_data_dir() {
                modules::logger::init(app_data_dir);
            }
            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
