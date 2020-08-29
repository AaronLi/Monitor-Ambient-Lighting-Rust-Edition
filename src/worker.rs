//The part of the program in charge of capturing the screen and printing to output
extern crate serialport;
extern crate scrap;

use std::{thread, time, sync};
use crate::framerate;
use crate::kernel::Kernel;
use std::io::Write;
use std::ops::Deref;
use crate::program_config::ProgramConfiguration;
use crate::monitor_config::MonitorConfiguration;

pub enum Error{
    OpenSerialError,
    OpenCapturerError,
}

pub struct Worker {
    open_serial_port: Box<dyn serialport::SerialPort>,
    display_capturer: scrap::Capturer,
    blur_kernel: Kernel,
    refreshrate: framerate::FramerateLimiter,
    pixel_locations: Vec<[usize; 2]>
}

impl Worker {
    pub fn new(program_config: ProgramConfiguration, monitor_config: MonitorConfiguration, b_kernel: Kernel, display_index: usize) -> Result<Worker, Error> {
        let display_config_info = monitor_config.monitors.get(display_index).unwrap();
        let display_capturer = scrap::Capturer::new(scrap::Display::all().unwrap().remove(display_config_info.monitor_number-1));
        let pixel_locations = monitor_config.get_pixel_locations(&b_kernel).unwrap();

        let open_serial_port = match program_config.get_open_serial_port(){
            Some(port) => port,
            None => return Err(Error::OpenSerialError)
        };

        let unwrapped_display_capturer = match display_capturer{
            Ok(capturer) => capturer,
            Err(_err) => return Err(Error::OpenCapturerError)
        };

        Ok(Worker{
            open_serial_port: open_serial_port,
            display_capturer: unwrapped_display_capturer,
            blur_kernel: b_kernel,
            refreshrate: program_config.get_refreshrate_controller(),
            pixel_locations: pixel_locations
        })
    }
    pub fn tick(&mut self){
        self.refreshrate.tick();
    }

    pub fn read_and_output(&mut self) {
        // locks on the display capturer and serial port should be acquireable with very little
        // blocking since the only time they're acquired elsewhere is for the purpose of modifying
        // the serial output mode and display capturer from the taskbar
        let captured_image: Vec<u8> = match self.display_capturer.frame() {
            Ok(frame) => frame.to_vec(),
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    // Wait until function is called again to try and capture another screenshot
                    thread::sleep(time::Duration::new(1, 0) / self.refreshrate.tick_rate as u32);
                }
                return;
            }
        };
        let mut output_colours = Vec::new();
        for point in self.pixel_locations.deref() {
            output_colours.extend_from_slice(&self.blur_kernel.kernel_pass_result(&captured_image, self.display_capturer.width(), self.display_capturer.height(), point[0], point[1]));
        };

        self.open_serial_port.write_all(output_colours.as_slice()).expect("Could not write to serial port");
    }

    /*pub fn set_serial_port(&mut self, new_serial_port: Box<dyn serialport::SerialPort>){
        let mut serial_port = self.open_serial_port.lock().unwrap();

        *serial_port = new_serial_port;
    }

    pub fn set_display_capturer(&mut self, new_capturer: scrap::Capturer){
        let mut capturer = self.display_capturer.lock().unwrap();

        *capturer = new_capturer;
    }

    pub fn set_blurring_kernel(&mut self, new_kernel: Kernel){
        let mut blur_kernel = self.blur_kernel.lock().unwrap();

        *blur_kernel = new_kernel;
    }*/
}