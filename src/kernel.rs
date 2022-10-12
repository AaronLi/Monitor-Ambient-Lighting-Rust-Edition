pub const MAX_KERNEL_SIZE: usize = 32*32;

fn get_pixel_address(x: usize, y: usize, image_width: usize, image_height: usize) -> Option<usize>{
    if y >= image_height || x >= image_width {
        return None
    }
    else {
        let row = image_width * y;
        let column = x;
        return Some((row + column) * 4);
    }
}

#[derive(Copy, Clone)]
pub struct Kernel{
    pub weights: [f32; MAX_KERNEL_SIZE],
    pub width: usize,
    pub height: usize,
    pub coefficient: f32,
}

impl Default for Kernel{
    fn default() -> Self {
        Kernel{
            weights: [1.0; MAX_KERNEL_SIZE],
            width: 1,
            height: 1,
            coefficient: 1.0
        }
    }
}

impl Kernel{
    /*pub fn gaussian(width: usize, height: usize, std_dev: f32) -> Kernel {
        Kernel{
            weights: vec![],
            width,
            height,
            coefficient: 0.0 // 1.0 / (2.0 * f32::PI() * std_dev.pow(2.0))
        };

        unimplemented!()
    }*/

    pub fn averaging(width: usize, height: usize) -> Kernel{
        Kernel{
            weights: [1.0; MAX_KERNEL_SIZE],
            width,
            height,
            coefficient: 1.0 / (width as f32 * height as f32)
        }
    }


    pub fn kernel_pass_result(&self, image_data: &[u8], image_width: usize, image_height: usize, kernel_apply_x: usize, kernel_apply_y: usize) -> [u8; 3]{
        let kernel_left_start = self.width / 2;
        let kernel_top_start = self.height / 2;

        let mut kernel_sum: [f32; 3] = [0_f32; 3];

        for k_y in 0..self.height{
            for k_x in 0..self.width{
                // Check subtraction won't wraparound for x and y
                let x_in_bounds = kernel_left_start <= (kernel_apply_x + k_x);
                let y_in_bounds = kernel_top_start <= (kernel_apply_y + k_y);
                if x_in_bounds && y_in_bounds {

                    let p_address = get_pixel_address(kernel_apply_x + k_x - kernel_left_start, kernel_apply_y + k_y - kernel_top_start, image_width, image_height);
                    match p_address {
                        None => {
                            // Don't add any values since it's all black
                        },
                        Some(address) => {
                            let (b, g, r, _a) = (image_data[address], image_data[address + 1], image_data[address + 2], image_data[address + 3]);

                            kernel_sum[0] += (r as f32) * self.weights[k_y * self.width + k_x];
                            kernel_sum[1] += (g as f32) * self.weights[k_y * self.width + k_x];
                            kernel_sum[2] += (b as f32) * self.weights[k_y * self.width + k_x];
                        }
                    };
                }
            }
        }
        kernel_sum[0] *= self.coefficient;
        kernel_sum[1] *= self.coefficient;
        kernel_sum[2] *= self.coefficient;

        let output = [kernel_sum[0] as u8, kernel_sum[1] as u8, kernel_sum[2] as u8];

        return output;
    }
}