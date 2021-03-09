//The part of the program in charge of capturing the screen and printing to output
extern crate serialport;
extern crate scrap;

use std::{thread, time, io};
use std::io::Write;
use std::ops::Deref;
use crate::core::kernel::Kernel;
use crate::core::{framerate, program_config};
use crate::core::monitor_config::MonitorConfiguration;

#[derive(Debug)]
pub enum Error{
    OpenSerialError,
    OpenCapturerError,
}

pub enum ControlMessages{
    StopWorker,
    UpdateConfiguration
}

pub struct Worker {
    pub open_serial_port: Box<dyn serialport::SerialPort>,
    pub display_capturer: scrap::Capturer,
    blur_kernel: Kernel,
    refreshrate: framerate::FramerateLimiter,
    pixel_locations: Vec<[usize; 2]>,
    captured_image: Vec<u8>
}


impl Worker {
    pub fn new(p_config: program_config::ProgramConfiguration, monitor_config: MonitorConfiguration, b_kernel: Kernel, display_index: usize) -> Result<Worker, Error> {
        let display_capturer = Worker::get_display_capturer(&monitor_config, display_index);
        let pixel_locations = monitor_config.get_pixel_locations(&b_kernel).unwrap();

        let open_serial_port = match p_config.get_open_serial_port(){
            Some(port) => port,
            None => {
                eprintln!("Failed to open serial port!");
                return Err(Error::OpenSerialError)
            }
        };

        let unwrapped_display_capturer = match display_capturer{
            Ok(capturer) => capturer,
            Err(_err) => {eprintln!("{}", _err); return Err(Error::OpenCapturerError)}
        };

        Ok(Worker{
            open_serial_port: open_serial_port,
            display_capturer: unwrapped_display_capturer,
            blur_kernel: b_kernel,
            refreshrate: p_config.get_refreshrate_controller(),
            pixel_locations: pixel_locations,
            captured_image: Vec::new()
        })
    }
    pub fn tick(&mut self){
        self.refreshrate.tick();
    }

    fn get_display_capturer(monitor_config: &MonitorConfiguration, display_index: usize) -> io::Result<scrap::Capturer>{
        let display_config_info = monitor_config.monitors.get(display_index).unwrap();
        scrap::Capturer::new(scrap::Display::all().unwrap().remove(display_config_info.monitor_number-1))
    }

    pub fn read_and_output(&mut self) {
        // locks on the display capturer and serial port should be acquireable with very little
        // blocking since the only time they're acquired elsewhere is for the purpose of modifying
        // the serial output mode and display capturer from the taskbar

        match self.display_capturer.frame() {
            Ok(frame) => self.captured_image = frame.to_vec(),
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    // Wait until function is called again to try and capture another screenshot
                    thread::sleep(time::Duration::new(1, 0) / self.refreshrate.tick_rate as u32);
                }
            }
        };
        if self.captured_image.len() == 0{
            return;
        }
        let mut output_colours = Vec::new();
        for point in self.pixel_locations.deref() {
            output_colours.extend_from_slice(&self.blur_kernel.kernel_pass_result(&self.captured_image, self.display_capturer.width(), self.display_capturer.height(), point[0], point[1]));
        };
        self.open_serial_port.write_all(output_colours.as_slice()).expect("Could not write to serial port");
    }

    pub fn update_settings(&mut self, p_config: Option<program_config::ProgramConfiguration>, monitor_config: Option<MonitorConfiguration>, conv_kernel: Option<Kernel>){
        if p_config.is_some() {
            let program_config_info = p_config.unwrap();
            match self.open_serial_port.name() {
                Some(name) => {
                    if name.to_uppercase() != program_config_info.serial_port {
                        // new serial port
                        match program_config_info.get_open_serial_port() {
                            Some(port) => self.open_serial_port = port,
                            None => { println!("Unable to open serial port") }
                        }
                    } else {
                        self.open_serial_port.set_all(&program_config_info.get_serial_port_settings()).unwrap();
                    }
                }
                None => { eprintln!("Failed to retrieve current port name") }
            }
        }
        if monitor_config.is_some() {
            match Worker::get_display_capturer(&monitor_config.unwrap(), 0) {
                Ok(cap) => {
                    self.display_capturer = cap;
                },
                Err(e) => match e {
                    _ => {
                        println!("{}", e);
                    }
                },
            }
        }

        self.blur_kernel =  if conv_kernel.is_some() {conv_kernel.unwrap()} else{self.blur_kernel};
    }
}