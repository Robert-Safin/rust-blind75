pub fn compress(chars: &mut Vec<char>) -> i32 {
    // left sits at index where replacement should happen
    let mut left = 0;
    // right runs forward until it reaches new char
    let mut right = 0;

    // parses chars once
    while right < chars.len() {
        // grab current char from right as it ends its loop at a new char
        let current_char = chars[right];

        // count how many consecutive chars there are
        // check if right is in bounds first before reading index value
        let mut count = 0;
        while right < chars.len() && chars[right] == current_char {
            right += 1;
            count += 1;
        }

        // unconditionally insert current char at left and bump left
        chars[left] = current_char;
        left += 1;

        // if count is more than 2, insert count at left, else do nothing.
        if count > 1 {
            // need to handle cases when count is more than 10 (multiple digits)
            // parse count to digits and insert each digit and bump left
            for digit in count.to_string().chars() {
                chars[left] = digit;
                left += 1;
            }
        }
    }
    // do not return chars.len() as chars was 'partially' mutated,
    // where head is compressed and tail is garbage. Left ends
    // at index where last compression mutation ocurred.
    left as i32
}
