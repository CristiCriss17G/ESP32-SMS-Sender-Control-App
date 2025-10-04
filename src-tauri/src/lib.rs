mod utils;

use log::debug;
use tauri::{Manager, State};
use tauri_plugin_blec::models::{BleDevice, ScanFilter};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use utils::structs::Result;

struct AppData {
    selected_device: Option<BleDevice>,
}

#[tauri::command]
async fn find_btle_devices(name: &str) -> Result<String> {
    // const CHARACTERISTIC_UUID: Uuid = uuid!("51FF12BB-3ED8-46E5-B4F9-D64E2FEC021B");
    // const DATA: [u8; 500] = [0; 500];
    // let handler = tauri_plugin_blec::get_handler().unwrap();
    // handler
    //     .send_data(CHARACTERISTIC_UUID, &DATA, WriteType::WithResponse)
    //     .await
    //     .unwrap();
    let handler = tauri_plugin_blec::get_handler().unwrap();
    let (tx, mut rx) = mpsc::channel(1);
    handler
        .discover(Some(tx), 1000, ScanFilter::None, false)
        .await
        .unwrap();
    while let Some(devices) = rx.recv().await {
        println!("Discovered {devices:?}");
    }
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

// Functions to get and set the selected device
#[tauri::command]
async fn get_selected_device(state: State<'_, Mutex<AppData>>) -> Result<Option<BleDevice>> {
    let state = state.lock().await;
    debug!("Selected device: {:?}", state.selected_device);
    Ok(state.selected_device.clone())
}

#[tauri::command]
async fn set_selected_device(
    state: State<'_, Mutex<AppData>>,
    device: Option<BleDevice>,
) -> Result<()> {
    let mut state = state.lock().await;
    debug!("Setting selected device: {:?}", device);
    state.selected_device = device;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }
    builder = builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_blec::init())
        .invoke_handler(tauri::generate_handler![
            find_btle_devices,
            get_selected_device,
            set_selected_device
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .build(),
                )?;
            }
            app.manage(Mutex::new(AppData {
                selected_device: None,
            }));
            Ok(())
        });
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
