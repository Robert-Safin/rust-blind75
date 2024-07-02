pub fn max_vowels(s: String, k: i32) -> i32 {
    // convert to vec for easy indexing
    let chars: Vec<char> = s.chars().collect();

    // keep track of max seen
    let mut max_count = i32::MIN;

    // manually fill the first window 0 to k
    let mut current_window = 0;
    for i in 0..k {
        if matches!(chars[i as usize], 'a' | 'e' | 'i' | 'o' | 'u') {
            current_window += 1;
        }
    }

    // update max
    max_count = current_window;

    // start iterating from k to end.
    for i in k..chars.len() as i32 {
        // check if incoming char is vowel
        if matches!(chars[i as usize], 'a' | 'e' | 'i' | 'o' | 'u') {
            current_window += 1;
        }

        // check if outgoing char is vowel
        if matches!(chars[(i - k) as usize], 'a' | 'e' | 'i' | 'o' | 'u') {
            current_window -= 1;
        }

        // update max
        max_count = std::cmp::max(max_count, current_window)
    }

    return max_count;
}
