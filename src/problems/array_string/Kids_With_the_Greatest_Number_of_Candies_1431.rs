pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut current_max = 0;
    let mut ans: Vec<bool> = Vec::new();

    // need to run 2 loops over candies, first time run loop over refs to keep ownership of candies
    // goal is to find max value in vector
    for n in candies.iter() {
        // try to update max value, dereferencing candy number
        current_max = std::cmp::max(current_max, *n);
    }

    // 'how many candies kid needs to exceed class max before extra candies given'
    let threshold = current_max - extra_candies;

    // can now give up ownership of candies
    for n in candies {
        if n >= threshold {
            ans.push(true)
        } else {
            ans.push(false)
        }
    }

    return ans;
}
