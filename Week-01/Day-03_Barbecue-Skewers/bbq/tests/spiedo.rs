use  bbq::dep::*;


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn peperoni (){
        assert_eq!((2, 3), skw_type(vec!["--oooo-ooo--",
                                        "--xx--x--xx--",
                                        "--o---o--oo--",
                                        "--xx--x--ox--",
                                        "--xx--x--ox--"]));
    }

    
    #[test]
    fn salsiccia (){
        assert_eq!((3,2),skw_type(vec![
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"]))

    }

    #[test]
    fn giorgione (){
        assert_eq!((0,5),skw_type(vec![
            "--xxxxxxxx--",
            "--xxxxxxxx--",
            "xxxxxxxx",
            "xxxxxxxx-x--",
            "xxxxxxxxxxxxxxxx"]))

    }
}
