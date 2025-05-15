use std::{path::PathBuf, sync::Mutex};
use tauri::{Builder, Manager, State};
use tauri_plugin_cli::CliExt;
use window_vibrancy::apply_vibrancy;
mod config;
mod layout;
mod search;

struct AppState {
    config_path: String,
    config: config::CuraConfig,
    layout_path: String,
    layout: layout::Layout,
    std_in: Vec<String>,
    delimiter: String,
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(desktop)]
    app.handle()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .expect("Failed to setup global shortcut plugin");

    let all_path = PathBuf::from("/");
    app.asset_protocol_scope()
        .allow_directory(&all_path, true)
        .unwrap();

    let mut app_state = AppState {
        config_path: "".to_string(),
        config: config::CuraConfig::default(),
        layout_path: "".to_string(),
        layout: layout::Layout::default(),
        std_in: get_std_in(),
        delimiter: "".to_string(),
    };

    get_cli_input(app, &mut app_state);
    match config::read_config(&app_state.config_path) {
        Ok(config) => {
            app_state.config = config;
        }
        Err(err) => (),
    };

    match layout::read_layout(&app_state.layout_path) {
        Ok(layout) => {
            app_state.layout = layout;
        }
        Err(err) => (),
    };

    let window = app.get_window("main").expect("main window not found");

    // Set window size
    window
        .set_size(tauri::LogicalSize::new(
            app_state.config.width,
            app_state.config.height,
        ))
        .expect("failed to set window size");

    window.center().expect("failed to center window");

    apply_vibrancy(
        &window,
        window_vibrancy::NSVisualEffectMaterial::HudWindow,
        Some(window_vibrancy::NSVisualEffectState::Active),
        Some(16.0),
    )
    .expect("Unsupported platform, 'apply_vibrancy' is only supported on macOS");

    app.manage(Mutex::new(app_state));

    Ok(())
}

fn get_std_in() -> Vec<String> {
    let mut stdin_content = Vec::new();

    for line in std::io::stdin().lines() {
        match line {
            Ok(line) => stdin_content.push(line),
            Err(err) => eprintln!("IO error: {}", err),
        }
    }

    stdin_content
}

fn get_cli_input(app: &mut tauri::App, app_state: &mut AppState) {
    match app.cli().matches() {
        // `matches` here is a Struct with { args, subcommand }.
        // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
        // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
        Ok(matches) => {
            let config = matches.args.get("config");
            if config.is_some() && config.unwrap().occurrences > 0 {
                app_state.config_path = config.unwrap().value.as_str().unwrap().to_string();
            }
            let layout = matches.args.get("layout");
            if layout.is_some() && layout.unwrap().occurrences > 0 {
                app_state.layout_path = layout.unwrap().value.as_str().unwrap().to_string();
            }
            let delimiter = matches.args.get("delimiter");
            if delimiter.is_some() && delimiter.unwrap().occurrences > 0 {
                app_state.delimiter = delimiter.unwrap().value.as_str().unwrap().to_string();
            }
        }
        Err(_) => {}
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0); // Exit with status code 0 (success)
}

#[tauri::command]
fn read_stdin(state: State<Mutex<AppState>>) -> Vec<String> {
    let state = state.lock().unwrap();
    state.std_in.clone()
}

#[tauri::command]
fn read_delimiter(state: State<Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    state.delimiter.clone()
}

#[tauri::command]
fn print(out: &str) {
    println!("{}", out)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_macos_permissions::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            print,
            read_stdin,
            exit_app,
            config::read_config_state,
            search::autodetect_search,
            layout::read_layout_state,
            read_delimiter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
