extern crate json;

use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;

struct LEDCount {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

struct Bezel {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

struct Monitor {
    monitor_number: usize,
    led_order: String,
    diagonal_size: f32,
    led_distribution: LEDCount,
    leds_per_inch: f32,
    bezel_thickness: Bezel,
}

pub struct MonitorConfiguration {
    monitors: Vec<Monitor>,
}

impl MonitorConfiguration {

    pub fn load_from_file(path: &str) -> MonitorConfiguration {
        MonitorConfiguration::try_load(path).unwrap_or_default()
    }

    fn try_load(path: &str) -> Option<MonitorConfiguration> {
        let filepath = Path::new(path);
        let mut read_file = File::open(filepath).ok()?;
        let mut file_contents = String::new();
        read_file.read_to_string(&mut file_contents).ok()?;
        let file_data = json::parse(file_contents.as_str()).ok()?;
        let monitor_configurations = &file_data["monitor_configuration"];
        let mut all_monitors = Vec::new();
        for i in 0..monitor_configurations.len() {
            let monitor_data = &monitor_configurations[i];

            let monitor_instance = Monitor {
                monitor_number: monitor_data["monitor"].as_usize()?,
                led_order: String::from(monitor_data["led_order"].as_str()?),
                diagonal_size: monitor_data["diagonal_size"].as_f32()?,
                led_distribution: LEDCount {
                    top: monitor_data["led_count"]["top"].as_usize()?,
                    left: monitor_data["led_count"]["left"].as_usize()?,
                    right: monitor_data["led_count"]["right"].as_usize()?,
                    bottom: monitor_data["led_count"]["bottom"].as_usize()?,
                },
                leds_per_inch: monitor_data["leds_per_inch"].as_f32()?,
                bezel_thickness: Bezel {
                    top: monitor_data["bezel_thickness"]["top"].as_f32()?,
                    left: monitor_data["bezel_thickness"]["left"].as_f32()?,
                    right: monitor_data["bezel_thickness"]["right"].as_f32()?,
                    bottom: monitor_data["bezel_thickness"]["bottom"].as_f32()?,
                },
            };
            all_monitors.push(monitor_instance);
        };
        let output = MonitorConfiguration {
            monitors: all_monitors,
        };
        Some(output)
    }
}
impl Default for MonitorConfiguration{
    fn default() -> Self {
        MonitorConfiguration {
            monitors: vec![
                Monitor {
                    monitor_number: 1,
                    led_order: String::default(),
                    diagonal_size: 0.0,
                    led_distribution: LEDCount::default(),
                    leds_per_inch: 0.0,
                    bezel_thickness: Bezel::default(),
                }
            ]
        }
    }
}

impl Default for LEDCount{
    fn default() -> Self {
        LEDCount{
            top: 0,
            left: 0,
            bottom: 0,
            right: 0
        }
    }
}

impl Default for Bezel{
    fn default() -> Self {
        Bezel{
            top: 0.0,
            left: 0.0,
            bottom: 0.0,
            right: 0.0
        }
    }
}

impl fmt::Display for MonitorConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out_string = String::from("MonitorConfiguration[");

        for (i, monitor) in self.monitors.iter().enumerate() {
            out_string.push_str(format!("{}", monitor).as_str());
            if self.monitors.len() > 0 && i < self.monitors.len() - 1 {
                out_string.push_str(", ");
            }
        };
        out_string.push(']');
        write!(f, "{}", out_string.as_str())
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Display{{Number: {}, led_order: {}, diagonal_size: {}, led_count: {}, leds_per_inch: {}, bezel_thickness: {}}}",
               self.monitor_number,
               self.led_order,
               self.diagonal_size,
               self.led_distribution,
               self.leds_per_inch,
               self.bezel_thickness
        )
    }
}

impl fmt::Display for LEDCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LEDCount(#LEDs){{Top: {}, Left: {}, Bottom: {}, Right: {}}}",
               self.top,
               self.left,
               self.bottom,
               self.right
        )
    }
}

impl fmt::Display for Bezel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bezel(Inches){{Top: {}\", Left: {}\", Bottom: {}\", Right: {}\"}}",
               self.top,
               self.left,
               self.bottom,
               self.right
        )
    }
}