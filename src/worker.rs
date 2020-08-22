//The part of the program in charge of capturing the screen and printing to output
extern crate serialport;
extern crate scrap;

use std::{thread, time, sync};
use crate::{debugging, framerate};
use crate::kernel;

struct Worker {
    open_serial_port: sync::Mutex<Box<dyn serialport::SerialPort>>,
    display_capturer: sync::Mutex<scrap::Capturer>,
    kernel: sync::Mutex<kernel::Kernel>,
    refreshrate: sync::Mutex<framerate::FramerateLimiter>
}


impl Worker {
    pub fn new(serial_port: Box<dyn serialport::SerialPort>, blur_kernel: kernel::Kernel, capturer: scrap::Capturer, framerate_limiter: framerate::FramerateLimiter) -> Worker{
        Worker{
            open_serial_port: sync::Mutex::new(serial_port),
            display_capturer: sync::Mutex::new(capturer),
            kernel: sync::Mutex::new(blur_kernel),
            refreshrate: sync::Mutex::new(framerate_limiter),

        }
    }

    pub fn read_and_output(&mut self) {
        // locks on the display capturer and serial port should be acquireable with very little
        // blocking since the only time they're acquired elsewhere is for the purpose of modifying
        // the serial output mode and display capturer from the taskbar
        let mut display_capturer = self.display_capturer.lock().unwrap();

        let captured_image: scrap::Frame = match (*display_capturer).frame() {
            Ok(frame) => frame,
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    // Wait until function is called again to try and capture another screenshot
                    thread::sleep(time::Duration::new(1, 0) / 60);
                }
                return;
            }
        };

        debugging::save_to_file(captured_image.to_vec(), display_capturer.width(), display_capturer.height(), "images/currentframe.png");
    }

    pub fn set_serial_port(&mut self, new_serial_port: Box<dyn serialport::SerialPort>){
        let mut serial_port = self.open_serial_port.lock().unwrap();

        *serial_port = new_serial_port;
    }

    pub fn set_display_capturer(&mut self, new_capturer: scrap::Capturer){
        let mut capturer = self.display_capturer.lock().unwrap();

        *capturer = new_capturer;
    }

    pub fn set_blurring_kernel(&mut self, new_kernel: kernel::Kernel){
        let mut blur_kernel = self.kernel.lock().unwrap();

        *blur_kernel = new_kernel;
    }
}