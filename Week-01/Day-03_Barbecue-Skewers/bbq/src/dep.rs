
// returns number of veggies and meat skewers.
pub fn skw_type(skw_array: Vec<&str>) -> (i32,i32){

    let mut veg = 0;
    let mut meat = 0;
    
    let skw_array_str: Vec<String> = skw_array
        .iter() //iterate over vector
        .map(|str| str.to_string())// map &str to a String
        .collect(); //collect mapped values

    for grill  in &skw_array_str{
       match grill.find("x") {
        Some(_) => meat +=1,
        _ => veg +=1
           
       }  
    }

    return (veg,meat);
}