use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut operations = 0;

    // store for previously seen numbers
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for &num in &nums {
        // given nums[i] and k, can calculate what number needs to be added
        // to nums[i] to match k, aka 'complement'
        let complement = k - num;

        // look for complement key in hashmap
        if let Some(&count) = counts.get(&complement) {
            // complement key exists, it was added in the past but it could have been used up

              // complement key's value has uses left.
              // bump operations.
              // decrement complement key's value.
            if count > 0 {
                operations += 1;
                *counts.get_mut(&complement).unwrap() -= 1;
            } else {
                // complement key's value has run out.
                // insert num key or (if key already exists) bump up.
                *counts.entry(num).or_insert(0) += 1;
            }
        // complement key does not exist, insert num key or (if key already exists) bump up
        } else {
            *counts.entry(num).or_insert(0) += 1;
        }
    }

    operations
}
