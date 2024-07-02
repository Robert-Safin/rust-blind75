pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    // keep track of how many replacements left
    let mut replaces = k;

    // left push up if right has not legal moves left
    let mut left = 0;

    // right moves ahead picking up 1s and replacing 0 (if replacements left)
    let mut right = 0;

    // tracking longest slice
    let mut longest = i32::MIN;

    // this ensures all possible legal slices are seen
    // and prevents both pointer going out of bounds.
    // There are 3 possible cases:
    while right != nums.len() && left != nums.len() {

        // 1. next digit is 1:
        // add it to current slice, push right onto next digit
        if nums[right] == 1 {
            right += 1;

        // 2. next digit is 0 and some replacements remain:
        // add it to current slice, push right onto next digit and decrement replacements
        } else if nums[right] == 0 && replaces > 0 {
            right += 1;
            replaces -= 1;

        // 3. next digit is 0 and NO replacements remain:
        // only choice is to push up left, hopefully giving a freeing a replacement
        } else {
            // shrink slice and 'refund' a use of replacement
            if nums[left] == 0 {
                left += 1;
                replaces += 1;
            // shrink slice, allowing future shrinks to 'refund' a replacement
            } else {
                left += 1;
            }
        }
        // slice length can be computed from 2 pointers
        // try to update max
        longest = std::cmp::max(longest, (right - left) as i32)
    }

    return longest;
}
