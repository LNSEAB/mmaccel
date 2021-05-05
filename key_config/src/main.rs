#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod editor;
mod error;
mod old_key_map;
mod popup_menu;
mod shortcut_list;
mod side_menu;

use application::*;
use bindings::wrapper::*;
use bindings::Windows::Win32::{
    Controls::*, DisplayDevices::*, FileSystem::*, Gdi::*, HiDpi::*, KeyboardAndMouseInput::*, MenusAndResources::*,
    Shell::*, SystemServices::*, WindowsAndMessaging::*,
};
use editor::*;
use error::*;
use key_map::*;
use old_key_map::OldKeyMap;
use popup_menu::*;
use shortcut_list::*;
use side_menu::*;
use log4rs::append::{
    console::ConsoleAppender,
    file::FileAppender,
};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn error(text: impl AsRef<str>) {
    message_box(
        None,
        text,
        "MMAccel キー設定",
        MESSAGEBOX_STYLE::MB_OK | MESSAGEBOX_STYLE::MB_ICONERROR,
    );
}

fn build_logger() -> Result<(), Box<dyn std::error::Error + 'static>> {
    const FORMAT: &str = "[{d(%Y-%m-%d %H:%M:%S%z)} {l} (({f}:{L}))] {m}\n";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(FORMAT)))
        .build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(FORMAT)))
        .append(false)
        .build("key_config.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("stdout").appender("file").build(log::LevelFilter::Debug))?;
    log4rs::init_config(config)?;
    Ok(())
}

fn main() {
    if let Err(e) = build_logger() {
        error(&format!("MMAccel キー設定のログを取れません ({})", e));
    }
    std::panic::set_hook(Box::new(|info| {
        let msg = if let Some(location) = info.location() {
            if let Some(s) = info.payload().downcast_ref::<&str>() {
                format!("panic!!! {} ({}:{})", s, location.file(), location.line())
            } else {
                format!("panic!!! ({}:{})", location.file(), location.line())
            }
        } else {
            "panic!!! unknown".into()
        };
        log::error!("{}", &msg);
        error(&msg);
        log::info!("key_config panic");
    }));
    log::info!("key_config start");
    wita::run(wita::RunType::Wait, Application::new).unwrap();
    log::info!("key_config end");
}
