mod test_kernel;
mod kernel;

extern crate scrap;

use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::{Duration, Instant};
use crate::kernel::KernelApply;

fn main() {
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
        println!("Finished in {} seconds", now.elapsed().as_secs());
        println!("Saving frame");
        let now = Instant::now();
        save_to_file(frame_out, width, height, format!("images/out{}.ppm", num_saved).as_str());
        println!("Saved frame {} in {} seconds", num_saved, now.elapsed().as_secs());
        num_saved += 1;
    }
}





fn save_to_file(image_data: Vec<u8>, image_width: usize, image_height: usize, filename: &str){
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
