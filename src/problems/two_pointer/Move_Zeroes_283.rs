pub fn move_zeroes(nums: &mut Vec<i32>) {
    // index for next swap
    let mut left = 0;

    for right in 0..nums.len() {
        // if num not a 0 place place it at left, bump left
        if nums[right] != 0 {
            nums.swap(left, right);
            left += 1;
        }
    }
}
