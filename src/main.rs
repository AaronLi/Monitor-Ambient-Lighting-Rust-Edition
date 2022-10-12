mod util;
mod framerate;
pub mod kernel;
pub mod worker;
pub mod baudrate;
pub mod settings_configurer;
pub mod app;
mod monitor_configurer;
mod monitor_config;
mod monitor_configurer_widget;
mod side;
mod program_config;

use std::{path, fs};
use iced::{Sandbox};
use clap::{arg, Parser};
use directories::ProjectDirs;
use crate::settings_configurer::SettingsConfigurer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    configure_program: bool
}

fn main() {
    let cli = Args::parse();
    if cli.configure_program {
        SettingsConfigurer::run(SettingsConfigurer::default_window_settings(Some("assets/icon.ico"))).expect("Unable to launch settings configurer");
    }
    let config_directory = ProjectDirs::from("com", "dumfing", "monitor-ambient-lighting-rs").expect("Platform not supported");
    println!("{:?}", config_directory.config_dir());
    let assets_directory = path::Path::new("assets");
    if !assets_directory.exists() {
        fs::create_dir_all(assets_directory).unwrap();
    }

    let taskbar_app = systray::Application::new().unwrap();
    let worker_command_channel = app::spawn_worker_thread();
    app::setup_application(taskbar_app, worker_command_channel)
        .wait_for_message().expect("Taskbar icon does not want to wait for messages");
}
