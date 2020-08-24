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

use std::io::ErrorKind::WouldBlock;
use std::{thread, sync};
use std::time::{Duration, Instant};
use crate::debugging::select_serial_port;

fn main() {
    let gaussian_kernel = kernel::Kernel{
        weights: vec![
            /*1.0, 2.0, 1.0,
            2.0, 4.0, 2.0,
            1.0, 2.0, 1.0*/
            1.0, 4.0, 7.0, 4.0, 1.0,
            4.0, 16.0, 26.0, 16.0, 4.0,
            7.0, 26.0, 41.0, 26.0, 7.0,
            4.0, 16.0, 26.0, 16.0, 4.0,
            1.0, 4.0, 7.0, 4.0, 1.0
        ],
        width: 5,
        height: 5,
        coefficient: 1.0/273.0
    };

    let mut taskbarapp = systray::Application::new().unwrap();
    app::setup_application(&mut taskbarapp);

    let m_config = monitor_config::MonitorConfiguration::load_from_file("assets/example_monitor_configuration.json");
    let p_config = program_config::ProgramConfiguration::load_from_file("assets/example_program_configuration.json");
    let pixel_locations = m_config.get_pixel_locations(&gaussian_kernel).unwrap();
    println!("{}", m_config);
    println!("{}", p_config);
    //select_serial_port(None);

    let display = scrap::Display::primary().expect("Couldn't find primary display");
    let mut capturer = scrap::Capturer::new(display).expect("Couldn't begin display capture");

    let (width, height) = (capturer.width(), capturer.height());

    let mut frames = Vec::new();
    let to_capture = 8;
    loop {
        match capturer.frame() {
            Ok(frame) =>{
                frames.push(frame.to_vec());
                println!("Captured {} frames",frames.len());
            },

            Err(error) => {
                if error.kind() == WouldBlock {
                    println!("Waiting");
                    thread::sleep(Duration::new(1, 0) / 144);
                    continue;
                }else{
                    panic!("Unknown error while capturing!");
                }
            }
        };

        if frames.len() >= to_capture {break};
    }

    let mut num_saved = 0;
    for f in frames {
        let mut frame_out = Vec::new();
        println!("Applying kernel");
        let now = Instant::now();
        for y in 0..height {
            for x in 0..width {
                let result = gaussian_kernel.kernel_pass_result(&f, width, height, x, y);
                frame_out.extend_from_slice(&result);
            }
        }
        for point in pixel_locations.clone(){
            let address = (point[0] + width * point[1]) * 3;

            frame_out[address] = 255;
            frame_out[address+1] = 0;
            frame_out[address+2] = 0;
        }
        println!("Finished in {} seconds", now.elapsed().as_secs());
        println!("Saving frame");
        let now = Instant::now();
        debugging::save_to_file(frame_out, width, height, format!("images/out{:03}.ppm", num_saved).as_str());
        println!("Saved frame {} in {} seconds", num_saved, now.elapsed().as_secs());
        num_saved += 1;
    }
}
