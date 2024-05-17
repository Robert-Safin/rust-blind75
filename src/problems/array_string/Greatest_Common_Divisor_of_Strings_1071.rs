use std::cmp;



pub fn gcd_of_strings(str1: String, str2: String) -> String {
    // if string joined strings to each other are not equal, there is no gcd
    if (str1.clone() + &str2) != (str2.clone() + &str1) {
        return String::new();
    }

    // greedy, start with max length of either string
    let mut len = cmp::max(str1.len(), str2.len());


    // at this point we know there is a gcd, so can use unconditional loop
    loop {
        // current len can perfectly divide both strings, break loop
        if str1.len() % len == 0 && str2.len() % len == 0 {
            break;
        } else {
        // try smaller len
          len -= 1
        }
    }

    // return slice of either input strings
    return str1[0..len].to_owned();
}
