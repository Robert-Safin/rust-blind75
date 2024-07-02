pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut current_altitude = 0;
    let mut max_altitude = 0;

    for point in gain {
        current_altitude += point;
        if current_altitude > max_altitude {
            max_altitude = current_altitude;
        }
    }

    max_altitude
}
