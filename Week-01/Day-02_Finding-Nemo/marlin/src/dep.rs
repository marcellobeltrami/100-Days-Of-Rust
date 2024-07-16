fn rm_sp_char(nm_loc:&str) -> String{
    nm_loc.chars().filter(|c| c.is_alphabetic() || c.is_whitespace() ).collect()
}

//Print location of word Nemo. 
pub fn nemo (nm_loc:&str)->i32{
   
    let mut counter = 0;
    let cleaned = rm_sp_char(nm_loc);
    println!("{}",cleaned);


    for i in cleaned.split_whitespace(){
        
        println!("{}",i);
        counter += 1;
        if i == "Nemo"{
            return counter;
        } else {}

    };
 
    return counter;
}