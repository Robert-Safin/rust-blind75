pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_len = 0;
    let mut zero_count = 0;


    while right < nums.len() {
        if nums[right] == 0 {
            zero_count += 1;
        }
        while zero_count > 1 {
            if nums[left] == 0 {
                zero_count -= 1;
            }
            left += 1;
        }
        max_len = max_len.max(right - left);
        right += 1;
    }
    max_len as i32
}
