extern crate json;

use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::{fmt, io};
use std::fmt::{Formatter, Display};
use crate::core::kernel;
use crate::core::side::{SideDirection, Side};

pub struct LEDCount {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

pub struct Bezel {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

pub struct LEDDirectionSequence {
    data: Vec<SideDirection>
}

impl Default for LEDDirectionSequence{
    fn default() -> Self {
        LEDDirectionSequence{
            data: vec![]
        }
    }
}

impl Display for LEDDirectionSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out_string = String::new();
        
        for side_direction in &self.data{
            if out_string.len() > 0{
                out_string.push_str(", ");
            }
            out_string.push_str(format!("From {} To {}", side_direction.side, side_direction.direction).as_str());
            
        }
        
        write!(
            f,
            "{}",
            out_string
        )
    }
}

pub struct Monitor {
    pub monitor_number: usize,
    pub led_order: LEDDirectionSequence,
    pub diagonal_size: f32,
    pub led_distribution: LEDCount,
    pub leds_per_inch: f32,
    pub bezel_thickness: Bezel,
}

pub struct MonitorConfiguration {
    pub monitors: Vec<Monitor>,
}

impl MonitorConfiguration {

    pub fn get_pixel_locations(&self, blend_kernel: &kernel::Kernel) -> Option<Vec<[usize; 2]>>{
        let mut output = Vec::new();
        let displays = scrap::Display::all().ok()?;

        for (monitor, monitor_pixel) in self.monitors.iter().zip(displays.iter()) {
            let bezel_length = (monitor.bezel_thickness.left + monitor.bezel_thickness.right).hypot(monitor.bezel_thickness.top + monitor.bezel_thickness.bottom);
            let physical_diagonal = bezel_length + monitor.diagonal_size;
            let kernel_diagonal = (blend_kernel.width as f32).hypot(blend_kernel.height as f32);
            // subtract the kernel diagonal so that the kernel can start fully within the image, this prevents only half of the kernel being used and producing poorer results
            // alternate solution: have coefficient be ~2 for kernel to compensate for half the kernel being black
            let pixel_diagonal = (monitor_pixel.width() as f32).hypot(monitor_pixel.height() as f32) - kernel_diagonal;

            let inch_pixel_ratio = physical_diagonal / pixel_diagonal;

            let pixels_per_led = monitor.leds_per_inch * (1.0 / inch_pixel_ratio);

            let _monitor_width = pixels_per_led * monitor_pixel.width() as f32;
            let _monitor_height = pixels_per_led * monitor_pixel.height() as f32;

            for side_direction in &monitor.led_order.data{
                let (side, direction) = (side_direction.side, side_direction.direction);
                let pixel_pos = MonitorConfiguration::get_starting_xy(side, direction, monitor_pixel, blend_kernel);

                let num_leds = match side {
                    Side::LEFT => monitor.led_distribution.left,
                    Side::RIGHT => monitor.led_distribution.right,
                    Side::TOP => monitor.led_distribution.top,
                    Side::BOTTOM => monitor.led_distribution.bottom,
                    Side::ERROR => 0
                };

                let step_amount = match direction {
                    Side::LEFT => [-pixels_per_led, 0.0],
                    Side::RIGHT => [pixels_per_led, 0.0],
                    Side::TOP => [0.0, -pixels_per_led],
                    Side::BOTTOM => [0.0, pixels_per_led],
                    Side::ERROR => [0.0, 0.0]
                };

                for led_number in 0..num_leds{
                    output.push([(pixel_pos[0] + (step_amount[0] * led_number as f32)).round() as usize, (pixel_pos[1] + (step_amount[1] * led_number as f32)).round() as usize]);
                };
            }
        };
        Some(output)
    }

    fn get_starting_xy(side: Side, direction: Side, screen: &scrap::Display, kernel_info : &kernel::Kernel) -> [f32; 2]{
        let mut output: [f32; 2] = [0.0, 0.0];
        let (half_kernel_width, half_kernel_height) = (kernel_info.width as f32/2.0, kernel_info.height as f32/2.0);
        let (screen_width, screen_height) = (screen.width() as f32, screen.height() as f32);
        match side {
            Side::LEFT => {
                output[0] = half_kernel_width;
            },
            Side::RIGHT => {
                output[0] = screen_width - half_kernel_width;
            },
            Side::TOP => {
                output[1] = half_kernel_height;
            },
            Side::BOTTOM => {
                output[1] = screen_height - half_kernel_height;
            },
            Side::ERROR => {}
        }

        match direction {
            Side::LEFT => {
                output[0] = screen_width - half_kernel_width;
            },
            Side::RIGHT => {
                output[0] = half_kernel_width;
            },
            Side::TOP => {
                output[1] = screen_height - half_kernel_height;
            },
            Side::BOTTOM => {
                output[1] = half_kernel_height;
            },
            Side::ERROR => {}
        }
        output
    }

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
                led_order: MonitorConfiguration::parse_led_order(monitor_data["led_order"].as_str()?).into(),
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

    fn parse_led_order(to_parse: &str) -> LEDDirectionSequence {
        let mut out = Vec::new();
        let mut parse_chars = to_parse.chars();
        loop {
            let side = match parse_chars.next() {
                None => {break},
                Some(c) => {c.into()},
            };

            let direction = match parse_chars.next(){
                None => {eprintln!("Terminated on direction block rather than side block"); break},
                Some(c) => {c.into()},
            };


            out.push(SideDirection{
                side: side,
                direction: direction
            });
        };

        LEDDirectionSequence{
            data: out,
        }
    }
}
impl Default for MonitorConfiguration{
    fn default() -> Self {
        MonitorConfiguration {
            monitors: vec![
                Monitor {
                    monitor_number: 1,
                    led_order: LEDDirectionSequence::default(),
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