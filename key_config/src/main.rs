#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod editor;
mod popup_menu;
mod shortcut_list;
mod side_menu;

use application::*;
use bindings::wrapper::*;
use bindings::Windows::Win32::{
    Controls::*, DisplayDevices::*, Gdi::*, HiDpi::*, KeyboardAndMouseInput::*, MenusAndResources::*, Shell::*,
    SystemServices::*, WindowsAndMessaging::*,
};
use editor::*;
use key_map::*;
use popup_menu::*;
use shortcut_list::*;
use side_menu::*;

fn main() {
    env_logger::init();
    wita::run(wita::RunType::Wait, Application::new).unwrap();
}
