use crate::kernel;
use crate::kernel::KernelApply;

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn kernel_test(){
        let k = kernel::Kernel{
            weights: vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            width: 3,
            height: 3,
            coefficient: 1.0
        };
        assert_eq!(1, 1);
    }

}