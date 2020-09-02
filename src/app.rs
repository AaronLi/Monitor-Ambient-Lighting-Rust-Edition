extern crate systray;
extern crate winit;

use std::{thread, env, sync};
use crate::{kernel, program_config, monitor_config, worker};
use std::process::Command;
use std::sync::{Mutex, Arc};
use crate::worker::{Worker, Error, ControlMessages};

#[derive(Clone)]
pub struct WorkerControl {
    worker_communicator: sync::mpsc::Sender<worker::ControlMessages>,
}

impl WorkerControl{

    pub fn new() -> WorkerControl{
        let conv_kernel = kernel::Kernel::averaging(12, 12);
        let p_config = program_config::ProgramConfiguration::load_from_file("assets/program_configuration.json");
        let m_config = monitor_config::MonitorConfiguration::load_from_file("assets/monitor_configuration.json");
        let (tx, rx) = sync::mpsc::channel();
        let comm_endpoint = WorkerControl{
            worker_communicator: tx,
        };
        thread::spawn(move || {
            let mut test_worker = match Worker::new(p_config, m_config, conv_kernel, 0) {
                Ok(worker_inst) => worker_inst,
                Err(error) => {
                    eprintln!("Could not intialize worker");
                    match error{
                        worker::Error::OpenCapturerError =>{eprintln!("Unable to open display capturer")},
                        Error::OpenSerialError => {eprintln!("Unable to open serial port")}
                    }
                    return
                }
            };
            println!("Running");
            loop {
                match rx.recv() {
                    Ok(message) => {
                        match message{
                            worker::ControlMessages::StopWorker => {println!("stopping");break},

                            ControlMessages::UpdateConfiguration => {
                                let p_config = program_config::ProgramConfiguration::load_from_file("assets/program_configuration.json");
                                let m_config = monitor_config::MonitorConfiguration::load_from_file("assets/monitor_configuration.json");
                                println!("{}\n{}", p_config, m_config);
                                test_worker.update_settings(Some(p_config), None, None);
                                println!("Updated configurations");
                            }
                        }
                    },
                    Err(_e) => break,
                };
                test_worker.read_and_output();
                test_worker.tick();
            }
        });
        comm_endpoint
    }
}

pub fn setup_application(app: &mut systray::Application, worker_controller: Arc<Mutex<WorkerControl>>){
    // The app won't do any events unless you tell it to wait for messages
    // TODO: run monitor scanning and output segments in separate thread
    // app.quit() quits the taskbar process and it will have to be reconstructed
    let cwc = worker_controller.clone();
    app.set_icon_from_file("assets/icon.ico").expect("Unable to set icon for menu");

    app.add_menu_item("Configure", move |_application| {
        Command::new(env::current_exe().unwrap().to_str().unwrap()).arg("configure_program").output().unwrap();
        cwc.lock().unwrap().worker_communicator.send(worker::ControlMessages::UpdateConfiguration).unwrap();
        Ok::<_, systray::Error>(())
    }).expect("Unable to add configure button to menu");

    let cwc2 = worker_controller.clone();

    app.add_menu_item("Quit", move |application| {
        cwc2.lock().unwrap().worker_communicator.send(worker::ControlMessages::StopWorker).unwrap();
        application.quit();
        Ok::<_, systray::Error>(())
    }).expect("Unable to add quit button to menu");
}