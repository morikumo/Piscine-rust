pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let mut i = 0;
    while i < pattern.len() && pattern[i] != b'*' {
        if i >= query.len() || pattern[i] != query[i] {
            return false;
        }
        i += 1;
    }
    if i == pattern.len() {
        return i == query.len();
    }
    i += 1;
    while i < pattern.len() && pattern[i] == b'*' {
        i += 1;
    }
    if i == pattern.len() {
        return true;
    }
    let mut j = 0;
    while j < query.len() {
        if strpcmp(&query[j..], &pattern[i..]) {
            return true;
        }
        j += 1;
    }
    false
}

pub fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32{
    if a < b {
        a
    }
    else {
        b
    }
}