extern crate systray;
extern crate winit;

use std::{thread, env, sync};
use std::process::Command;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Sender, TryRecvError};
use crate::kernel::Kernel;
use crate::monitor_config::MonitorConfiguration;
use crate::program_config::ProgramConfiguration;
use crate::worker::{ControlMessage, Error, Worker};


pub fn spawn_worker_thread() -> Arc<Mutex<Sender<ControlMessage>>>{
    let conv_kernel = Kernel::averaging(12, 12);
    let p_config = ProgramConfiguration::load_from_file("assets/program_configuration.json");
    let m_config = MonitorConfiguration::load_from_file("assets/monitor_configuration.json");
    let (tx, rx) = sync::mpsc::channel();
    thread::spawn(move || {
        let mut test_worker = match Worker::new(p_config, m_config, conv_kernel, 0) {
            Ok(worker_inst) => worker_inst,
            Err(error) => {
                eprintln!("Could not intialize worker");
                match error{
                    Error::OpenCapturerError =>{eprintln!("Unable to open display capturer")},
                    Error::OpenSerialError => {eprintln!("Unable to open serial port")}
                }
                return
            }
        };
        println!("Running");
        loop {
            match rx.try_recv() {
                Ok(message) => {
                    match message{
                        ControlMessage::StopWorker => {println!("stopping");break},

                        ControlMessage::UpdateConfiguration => {
                            let p_config = ProgramConfiguration::load_from_file("assets/program_configuration.json");
                            let m_config = MonitorConfiguration::load_from_file("assets/monitor_configuration.json");
                            println!("{}\n{}", p_config, m_config);
                            test_worker.update_settings(Some(p_config), None, None);
                            println!("Updated configurations");
                        }
                    }
                },
                Err(e) => {
                    match e{
                        TryRecvError::Empty => {}
                        TryRecvError::Disconnected => {break}
                    }
                },
            };
            test_worker.read_and_output();
            test_worker.tick();
        }
    });
    Arc::new(Mutex::new(tx))
}


pub fn setup_application(mut app: systray::Application, worker_controller: Arc<Mutex<Sender<ControlMessage>>>) -> systray::Application{
    // The app won't do any events unless you tell it to wait for messages
    // app.quit() quits the taskbar process and it will have to be reconstructed
    app.set_icon_from_file("assets/icon.ico").expect("Unable to set icon for menu");
    let worker_controller_copy = Arc::clone(&worker_controller);
    app.add_menu_item("Configure", move |_application| {
        Command::new(env::current_exe().unwrap().to_str().unwrap()).arg("configure_program").output().unwrap();
        worker_controller_copy.lock().unwrap().send(ControlMessage::UpdateConfiguration).unwrap();
        Ok::<_, systray::Error>(())
    }).expect("Unable to add configure button to menu");

    app.add_menu_item("Quit", move |application| {
        worker_controller.lock().unwrap().send(ControlMessage::StopWorker).unwrap();
        application.quit();
        Ok::<_, systray::Error>(())
    }).expect("Unable to add quit button to menu");

    app
}