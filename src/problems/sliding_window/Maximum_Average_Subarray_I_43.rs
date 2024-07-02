
// [Possible optimization]
// use conventional sliding window to + incoming number to window sum and
// - outgoing number. This avoids recalculating the sum of all number in the window,
// instead 1 number is added to sum and 1 number removed from sum.

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    // init sum with 'negative infinity'
    let mut sum = i32::MIN;

    // loop through windows of size k
    for window in nums.windows(k as usize) {
        // calculate sum of window
        let mut local_sum = 0;
        for n in window {
            local_sum += n;
        }
        // update max sum
        sum = std::cmp::max(sum, local_sum)
    }
    // return avg
    return sum as f64 / k as f64;
}
