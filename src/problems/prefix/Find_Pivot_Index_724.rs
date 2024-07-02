pub fn pivot_index(nums: Vec<i32>) -> i32 {
    // calc total sum of entire vec for removing prefix sum from total.
    let sum: i32 = nums.iter().sum();

    let mut running_sum = 0;

    for i in 0..nums.len() {
        // need to remove current element from sum to get the sum of the right side.
        if running_sum == sum - running_sum - nums[i] {
            return i as i32;
        }
        running_sum += nums[i];
    }

    return -1;
}
