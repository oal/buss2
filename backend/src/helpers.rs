pub fn get_last_as_i32(value: &str) -> i32 {
    return value.split(':').last().unwrap().parse().unwrap();
}
