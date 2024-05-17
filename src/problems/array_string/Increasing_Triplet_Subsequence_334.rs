// Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.



pub fn increasing_triplet(nums: Vec<i32>) -> bool {

    let mut first = i32::MAX;
    let mut second = i32::MAX;

    for i in nums {
        if i <= first {
            first = i;
        } else if i <= second {
            second = i;
        } else {
            return true;
        }
    }

    return false;
}

//  20, 100, 10, 12, 5, 13,
