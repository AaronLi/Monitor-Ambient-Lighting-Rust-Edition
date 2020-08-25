use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{stdin, Write, stdout};
use serialport::{SerialPortInfo, SerialPort, SerialPortSettings};

pub fn _save_to_file(image_data: Vec<u8>, image_width: usize, image_height: usize, filename: &str){
    let path = Path::new(filename);
    path.extension().expect("Invalid file extension!");

    match fs::create_dir_all(path.parent().expect("Unable to extract parent from given path")){
        Err(err) => {
            panic!("{}", err);
        },
        Ok(_res) => {}
    }

    let display_text = path.display();

    let mut out_file = match File::create(path){
        Err(why) => panic!("Couldn't create {}: {}", display_text, why),
        Ok(file) => file,
    };
    out_file.write(format!("P6\n{} {} {}\n", image_width, image_height, 255).as_bytes()).unwrap();
    let mut out_bytes: Vec<u8> = Vec::new();
    for y in 0..image_height {
        for x in 0..image_width {
            let column = y * image_width;
            let address  = (column + x) * 3;
            let rgb: [u8; 3] = [image_data[address], image_data[address+1], image_data[address+2]];

            out_bytes.extend_from_slice(&rgb);
        }
    }
    out_file.write(out_bytes.as_slice()).unwrap();
}

pub fn _select_serial_port(settings: Option<SerialPortSettings>) -> Box<dyn SerialPort> {
    loop {
        let mut portnames = Vec::new();
        let available_ports: Vec<SerialPortInfo>  = match serialport::available_ports(){
            Ok(port_list) => port_list,
            Err(_error) => {
                continue;
            }
        };
        if available_ports.is_empty() {
            println!("No ports available.");
        }else {
            println!("Ports:");
            for (i, port) in available_ports.iter().enumerate() {
                println!("{}: {}", i + 1, port.port_name);
                portnames.push(port);
            }
        }
        print!("Select a port: ");
        let _ = stdout().flush();

        let mut selection = String::new();
        let _read_bytes = stdin().read_line(&mut selection);
        selection.pop();

        let serial_port_selection = match selection.parse::<usize>(){
            Ok(selection) => {
                if 0 < selection && selection < portnames.len() {
                    println!("{}", selection - 1);
                    portnames.get(selection - 1).unwrap()
                }else{
                    println!("Invalid selection");
                    continue;
                }
            },
            Err(_error) => {
                println!("Invalid selection");
                continue;
            }
        };

        return match settings {
            // If no settings are provided
            None => match serialport::open(&serial_port_selection.port_name){
                Ok(port_box) => {
                    port_box
                },
                Err(_error) => {
                    println!("Unable to open port");
                    continue;
                }
            },
            // If settings are provided
            Some(setting) => match serialport::open_with_settings(&serial_port_selection.port_name, &setting){
                Ok(port_box) => {
                    port_box
                },
                Err(_error) => {
                    println!("Unable to open port");
                    continue;
                }
            }

        }
    }
}