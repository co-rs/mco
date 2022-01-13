/// Index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
pub fn index(s: &str, substr: &str) -> i32 {
    match s.find(substr) {
        None => { -1 }
        Some(v) => { v as i32 }
    }
}