mod util;
mod core;

extern crate scrap;
extern crate serialport;
extern crate systray;
extern crate clap;

use std::{path, fs};
use std::sync::{Mutex, Arc};
use iced::{Sandbox};
use crate::core::{settings_configurer, app};

fn main() {
    let matches = clap::App::new("Monitor Ambient Lighting")
        .version("1.0")
        .author("Aaron Li")
        .about("A Rust based frontend for Monitor Ambient Lighting")
        .subcommand(clap::SubCommand::with_name("configure_program")
        ).get_matches();

    match matches.subcommand_matches("configure_program") {
        None => {}
        Some(_matches) => {
            settings_configurer::SettingsConfigurer::run(settings_configurer::SettingsConfigurer::default_window_settings(Some("assets/icon.ico"))).expect("Unable to launch settings configurer");
        }
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
