use std::{thread::sleep, time::Duration, option::Option };
use tauri::{Manager, LogicalSize, Size, Window};
use crate::APP;

pub fn selection_translate() {
    if let Some(window) = get_window() {
        let window_size = LogicalSize::new(350, 400);
        window.set_size(Size::new(window_size)).unwrap();

        window.set_always_on_top(true).unwrap();
        window.center().unwrap();
        window.show().unwrap();
        window.center().unwrap();
        sleep(Duration::from_millis(10));
        window.set_focus().unwrap();
    }
}

pub fn ocr_translate() {
    println!("ocr_translate")
}

pub fn get_window() -> Option<Window> {
    let app_handle = APP.get().unwrap();
    let window: Option<Window> = app_handle.get_window("translate");
    window
}

#[tauri::command]
pub fn hide_translate_window() {
    if let Some(window) = get_window() {
        window.hide().unwrap();
    }
}