
pub trait KernelApply{
    fn kernel_pass_result(&self, image_data: &Vec<u8>, image_width: usize, image_height: usize, kernel_apply_x: usize, kernel_apply_y: usize) -> [u8; 3];
}

pub struct Kernel{
    pub weights: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub coefficient: f32,
}

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

impl KernelApply for Kernel{
    fn kernel_pass_result(&self, image_data: &Vec<u8>, image_width: usize, image_height: usize, kernel_apply_x: usize, kernel_apply_y: usize) -> [u8; 3]{
        let kernel_left_start = (self.width / 2);
        let kernel_top_start = (self.height / 2);

        let mut kernel_sum: [f32; 3] = [0_f32; 3];

        for kY in 0..self.height{
            for kX in 0..self.width{
                // Check subtraction won't wraparound for x and y
                let x_in_bounds = (kernel_left_start <= (kernel_apply_x + kX));
                let y_in_bounds = (kernel_top_start <= (kernel_apply_y + kY));
                if x_in_bounds & y_in_bounds {

                    let p_address = get_pixel_address(kernel_apply_x + kX - kernel_left_start, kernel_apply_y + kY - kernel_top_start, image_width, image_height);
                    match p_address {
                        None => {
                            // Don't add any values since it's all black
                        },
                        Some(address) => {
                            let (b, g, r, _a) = (image_data[address], image_data[address + 1], image_data[address + 2], image_data[address + 3]);

                            kernel_sum[0] += (r as f32) * self.weights[kY * self.width + kX];
                            kernel_sum[1] += (g as f32) * self.weights[kY * self.width + kX];
                            kernel_sum[2] += (b as f32) * self.weights[kY * self.width + kX];
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