use  marlin::dep::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nemo_1() {
        assert_eq!(4, nemo("I am finding Nemo !"));
    }

    
    #[test]
    fn nemo_2() {
        assert_eq!(2, nemo("I Nemo am"));
    }

    
    #[test]
    fn nemo_3() {
        assert_eq!(1, nemo("Nemo is me"));
    }
}