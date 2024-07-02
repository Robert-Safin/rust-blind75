use std::collections::HashMap;
use std::collections::HashSet;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in arr {
        if map.contains_key(&n) {
            *map.get_mut(&n).unwrap() += 1;
        } else {
            map.insert(n, 1);
        }
    }

    let mut set: HashSet<i32> = HashSet::new();

    for v in map.values() {
        if set.contains(v) {
            return false;
        } else {
            set.insert(*v);
        }
    }

    return true;
}
