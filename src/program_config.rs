extern crate json;

use std::fmt::{Display, Formatter, Result};
use std::{path, fs};
use std::io::Read;

pub struct ProgramConfiguration {
    serial_port: String,
    refresh_rate: f32,
    baudrate: usize,
}

impl Default for ProgramConfiguration {
    fn default() -> Self {
        ProgramConfiguration {
            serial_port: String::from("COM0"),
            refresh_rate: 20.0,
            baudrate: 115200
        }
    }
}

impl Display for ProgramConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ProgramConfig{{serial_port: {}, refresh_rate: {}, baudrate: {}}}", self.serial_port, self.refresh_rate, self.baudrate)
    }
}

impl ProgramConfiguration {
    pub fn load_from_file(path_in: &str)-> ProgramConfiguration {
        ProgramConfiguration::try_load(path_in).unwrap_or_default()
    }

    fn try_load(path_in: &str) -> Option<ProgramConfiguration>{
        let file_path = path::Path::new(path_in);
        let mut open_file = fs::File::open(file_path).ok()?;

        let mut file_contents = String::new();
        open_file.read_to_string(&mut file_contents);

        let parsed_json = json::parse(file_contents.as_str()).ok()?;

        let out_config = ProgramConfiguration {
            serial_port: String::from(parsed_json["serial_port"].as_str()?).to_lowercase(),
            refresh_rate: parsed_json["refresh_rate"].as_f32()?,
            baudrate: parsed_json["baud_rate"].as_usize()?
        };
        Some(out_config)
    }
}