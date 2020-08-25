mod program_config;
mod monitor_config;
mod worker;
mod framerate;
mod app;
mod debugging;
mod test_kernel;
mod kernel;

extern crate scrap;
extern crate serialport;
extern crate systray;

use std::{path, fs, thread};
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let assets_directory = path::Path::new("assets");
    if !assets_directory.exists(){
        fs::create_dir_all(assets_directory);
    }

    let conv_kernel = kernel::Kernel::averaging(12, 12);

    let mut taskbarapp = systray::Application::new().unwrap();
    app::setup_application(&mut taskbarapp);
    //TODO: create method of declaring which monitor and program config file you wish to use
    let m_config = monitor_config::MonitorConfiguration::load_from_file("assets/example_monitor_configuration.json");
    let p_config = program_config::ProgramConfiguration::load_from_file("assets/example_program_configuration.json");
    let pixel_locations = m_config.get_pixel_locations(&conv_kernel).unwrap();
    println!("{}", m_config);
    println!("{}", p_config);
    println!(
        "Blur filter has an area of {} pixels, with {} locations there will be about {} * x operations",
        conv_kernel.weights.len(), pixel_locations.len(), pixel_locations.len() * conv_kernel.weights.len()
    );
    //select_serial_port(None);
    let (tx, rx) = mpsc::channel();
    let worker_thread = thread::spawn(move ||{
        let mut test_worker = Box::new(match worker::Worker::new(p_config, m_config, conv_kernel, 0){
            Ok(worker_inst) => worker_inst,
            Err(error) => panic!(error)
        });
        println!("Running");
        loop{
            let _received_message = match rx.recv() {
                Ok(message) => message,
                Err(_error) => break
            };
            //println!("{}", _received_message);
            test_worker.read_and_output();
            test_worker.tick();
        }
    });

    taskbarapp.wait_for_message().expect("Taskbar icon does not want to wait for messages");

}
