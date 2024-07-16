use std::collections::HashMap; // Import the HashMap type from the standard library


// takes in a string, counts pairs with hashmap and returns number of pairs.
pub fn pairing(socks: &str) -> usize {
    let socks_str = socks.to_string();

    if socks_str.len() == 0 {
        return 0;
    } else {
        let mut char_frequency: HashMap<char, usize> = HashMap::new();
        for i in socks_str.chars() {
            let count = char_frequency.entry(i).or_insert(0);
            *count += 1;
        }

        println!("{:?}",char_frequency);
        let mut counter = 0;
        for (_key, value) in &char_frequency {
            if value % 2 == 0 {
                
                counter += value/2;
            }
        }

        return counter;
    }
}

