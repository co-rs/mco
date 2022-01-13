/// Index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
pub fn index(s: &str, substr: &str) -> i32 {
    match s.find(substr) {
        None => { -1 }
        Some(v) => { v as i32 }
    }
}

// IndexFunc returns the index into s of the first Unicode
// code point satisfying f(c), or -1 if none do.
pub fn index_func(s: &str, f: fn(char) -> bool) -> i32 {
    return index_func_impl(s, f, true);
}

// indexFunc is the same as IndexFunc except that if
// truth==false, the sense of the predicate function is
// inverted.
fn index_func_impl(s: &str, f: fn(char) -> bool, truth: bool) -> i32 {
    let mut i = 0;
    for r in s.chars() {
        if (f)(r) == truth {
            return i;
        }
        i += 1;
    }
    return -1;
}