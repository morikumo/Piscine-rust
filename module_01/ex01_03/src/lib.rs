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

pub const fn color_name<'a>(color: &'a [u8; 3]) -> &'static str {
    match color {
        &[0, 0, 0] => "Pure black", 
        &[255, 255, 255] => "Pure white",
        &[255, 0, 0] => "Pure Red",
        &[0, 255, 0] => "Pure Green",
        &[0, 0, 255] => "Pure Blue",
        &[128, 128, 128] => "Pure Grey",
        &[0..=30,0..=30,0..=30] => "Almost black",
        &[128..=255,0..=127,0..=127] => "Reddish",
        &[0..=127,128..=255,0..=127] => "Greenish",
        &[0..=127,0..=127,128..=255] => "Bluish",
        _ => "unknown"
    }
}

pub fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {    
    if haystack.is_empty() || needle.is_empty() {
        return &[];
    }
    
    let mut best_start = 0; 
    let mut best_len = 0;   
    
    let mut current_start = None;
    let mut current_len = 0; 
    
    for i in 0..haystack.len() {
        if needle.contains(&haystack[i]) {
            if current_start.is_none() {
                current_start = Some(i);
            }
            current_len += 1;
        } else {
            if let Some(start) = current_start {
                if current_len > best_len {
                    best_start = start;
                    best_len = current_len;
                }
                current_start = None;
                current_len = 0;
            }
        }
    }
    
    if let Some(start) = current_start {
        if current_len > best_len {
            best_start = start;
            best_len = current_len;
        }
    }
    
    &haystack[best_start..best_start + best_len]
}
