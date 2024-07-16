
//prints amount of increased running distances.
pub fn runner(sat_runs:Vec<i32>) -> i32{
    
    let mut counter = 0;
    
    let mut i = 0;
    let mut j = 1;

    for _ in 0..(sat_runs.len()-1){
        
        if sat_runs[j] >   sat_runs[i]{
            counter +=1;
        }

        i +=1;
        j +=1;
    }

    counter
}