use tauri::{Manager, Runtime};
use window_shadows::set_shadow;

pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
    let window = match app.get_window("main") {
        Some(window) => window,
        None => {
            println!("Customization window not found!");
            return;
        }
    };
    set_shadow(&window, true).expect("Unsupported platform!");
}
