#[cfg(test)]
mod tests{

    use crate::kernel;

    #[test]
    fn kernel_test(){
        let _k = kernel::Kernel{
            weights: [1.0; kernel::MAX_KERNEL_SIZE],
            width: 3,
            height: 3,
            coefficient: 1.0
        };
        assert_eq!(1, 1);
    }

}