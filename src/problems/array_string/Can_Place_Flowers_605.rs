
// [possible micro optimizations]
// 1. instead of performing mutation 'flowerbed[i] = 1', update i by 2
// 2. break loop early if flower_left_to_plant is 0, as it is possible to plant more flowers than n



pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flower_left_to_plant = n;

    let mut i = 0;

    // while i is in bounds
    while i < flowerbed.len() {
        // check current, left and right plots are empty. However we cant simply check that all three indices are empty,
        // since e.g. in [0,0,1] the first plot is valid, we can know this as the plot to the left is out of bounds
            // current plot:
        if flowerbed[i] == 0
            // left plot: first check if left plot is out of bounds (valid "empty" plot)
            && (i == 0 || flowerbed[i - 1] == 0)
            // right plot: first check if right plot is out of bounds (valid "empty" plot)
            && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
        {
            // update plot vector, so future iterations can properly determine valid plots
            flowerbed[i] = 1;
            // decrement flowers_left
            flower_left_to_plant -= 1;
        }
        // bump i
        i += 1;
    }

    // it is possible to 'overplant' going negative, check for being 0 or less
    if flower_left_to_plant <= 0 {
        return true;
    } else {
        return false;
    }
}
