use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map_1: HashSet<i32> = HashSet::new();
    let mut map_2: HashSet<i32> = HashSet::new();

    for n in nums1 {
        map_1.insert(n);
    }

    for n in nums2 {
        map_2.insert(n);
    }

    let mut distinct_1: Vec<i32> = Vec::new();
    let mut distinct_2: Vec<i32> = Vec::new();

    for n in map_1.iter() {
        if !map_2.contains(&n) {
            distinct_1.push(*n);
        };
    }

    for n in map_2.iter() {
        if !map_1.contains(&n) {
            distinct_2.push(*n);
        };
    }

    let mut out: Vec<Vec<i32>> = Vec::new();

    out.push(distinct_1);
    out.push(distinct_2);

    return out;
}
