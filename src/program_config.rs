extern crate json;
extern crate serialport;

use std::fmt::{Display, Formatter, Result};
use std::{path, fs};
use std::io::{Read, Write};
use json::object;
use serialport::{SerialPort, DataBits, StopBits, Parity, FlowControl};
use crate::baudrate::Baudrate;
use crate::framerate::FramerateLimiter;

#[derive(Clone, PartialEq)]
pub struct ProgramConfiguration {
    pub serial_port: String,
    pub refresh_rate: f32,
    pub baudrate: Baudrate,
}

impl Default for ProgramConfiguration {
    fn default() -> Self {
        ProgramConfiguration {
            serial_port: String::from("COM0"),
            refresh_rate: 20.0,
            baudrate: Baudrate::default()
        }
    }
}

impl Display for ProgramConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ProgramConfig{{serial_port: {}, refresh_rate: {}, baudrate: {}}}", self.serial_port, self.refresh_rate, self.baudrate)
    }
}

impl ProgramConfiguration {
    pub fn get_open_serial_port(&self) -> Option<Box<dyn SerialPort>> {
        let settings = self.get_serial_port_settings();

        serialport::open_with_settings(self.serial_port.as_str(), &settings).ok()
    }

    pub fn get_serial_port_settings(&self) -> serialport::SerialPortSettings {
        serialport::SerialPortSettings{
            baud_rate: self.baudrate as u32,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Default::default()
        }
    }

    pub fn get_refreshrate_controller(&self) -> FramerateLimiter {
        FramerateLimiter::new(self.refresh_rate)
    }

    pub fn load_from_file(path_in: &str)-> ProgramConfiguration {
        ProgramConfiguration::try_load(path_in).unwrap_or_default()
    }

    fn try_load(path_in: &str) -> Option<ProgramConfiguration>{
        let file_path = path::Path::new(path_in);
        let mut open_file = fs::File::open(file_path).ok()?;
        let mut file_contents = String::new();
        open_file.read_to_string(&mut file_contents).expect(format!("Could not read file {}", path_in).as_str());

        let parsed_json = json::parse(file_contents.as_str()).ok()?;

        let out_config = ProgramConfiguration {
            serial_port: String::from(parsed_json["serial_port"].as_str()?),
            refresh_rate: parsed_json["refresh_rate"].as_f32()?,
            baudrate: Baudrate::from(parsed_json["baud_rate"].as_u32()?)
        };
        Some(out_config)
    }

    pub fn save_to_file(&self, path_in: &str){
        let file_path = path::Path::new(path_in);
        let mut open_file = fs::File::create(file_path).unwrap();

        let json_out = object!{
            "refresh_rate": self.refresh_rate,
            "baud_rate": self.baudrate as u32,
            "serial_port": self.serial_port.as_str()
        };
        println!("{}", json_out.to_string());
        open_file.write(json_out.pretty(4).as_bytes()).unwrap();
    }
}