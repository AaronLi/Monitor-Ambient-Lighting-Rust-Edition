mod program_config;
mod monitor_config;
mod worker;
mod framerate;
mod app;
mod debugging;
mod test_kernel;
mod kernel;
mod settings_configurer;
mod baudrate;

extern crate scrap;
extern crate serialport;
extern crate systray;

use std::{path, fs, env};
use std::sync::{Mutex, Arc};
use iced::{Sandbox};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1{
        match (&args[1]).as_str() {
            "configure_program" => settings_configurer::SettingsConfigurer::run(settings_configurer::SettingsConfigurer::default_window_settings(Some("assets/icon.ico"))),
            _ => {
                println!("Unknown parameter {}", &args[1]);
            }
        };
    }

    let assets_directory = path::Path::new("assets");
    if !assets_directory.exists(){
        fs::create_dir_all(assets_directory).unwrap();
    }

    let mut taskbarapp = systray::Application::new().unwrap();
    let worker_controller = Arc::new(Mutex::new(app::WorkerControl::new()));
    app::setup_application(&mut taskbarapp, worker_controller.clone());

    taskbarapp.wait_for_message().expect("Taskbar icon does not want to wait for messages");
}
