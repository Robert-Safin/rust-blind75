pub fn max_area(height: Vec<i32>) -> i32 {
    // init pointer at first and last indices (wall heights)
    let mut left = 0;
    let mut right = height.len() - 1;

    // global var to update max water 'volume'
    let mut max_water = 0;

    // keeps both pointers in bounds
    while left < right {
        // the 'volume' calculation is limited by the height of the shorter wall.
        // take the shorter of the two.
        let shorter_wall = std::cmp::min(height[left], height[right]);

        // get distance between 2 walls
        let distance = right - left;

        // calculate 'volume'
        let water_volume = shorter_wall * distance as i32;

        // update max_water if new water_volume is greater then previous water_volume
        max_water = std::cmp::max(max_water, water_volume);

        // when moving wall pointers, want to only shift the smaller wall inwards
        if height[left] < height[right] {
            left += 1;
        // also handle when height[left] == height[right], as there would be inf loop
        } else {
            right -= 1;
        }
    }

    return max_water;
}
