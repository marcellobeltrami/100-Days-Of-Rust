use johnny::dep::*;

#[cfg(test)]

mod test{

    use super::*;

    #[test]
    fn test_1 (){

        assert_eq!(3,runner(vec![10, 11, 12, 9, 10]));
    }

    
    #[test]
    fn test_2(){

        assert_eq!(1,runner(vec![6, 5, 4, 3, 2, 9]));

    }

    #[test]
    fn test_3(){
        assert_eq!(0,runner(vec![9, 9]));

    }


}