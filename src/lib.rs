// extern crate libc;

// use std::c_str::CString;
// use libc::c_char;

#[no_mangle]
pub extern "C" fn test_function() -> i32 {
    println!("Called the rust function.");
    0
}

// For F, G, H, I, below, see : https://en.wikipedia.org/wiki/MD5#Algorithm
// Note to self: all operators below are bitwise
// Also, ^ is XOR

/// F
#[no_mangle]
pub extern "C" fn f0(abcd: &[u32]) -> u32 {
    (abcd[1] & abcd[2]) | (!abcd[1] & abcd[3])
}

/// G
#[no_mangle]
pub extern "C" fn f1(abcd: &[u32]) -> u32 {
    (abcd[3] & abcd[1]) | (!abcd[3] & abcd[2])
}

/// H
#[no_mangle]
pub extern "C" fn f2(abcd: &[u32]) -> u32 {
    abcd[1] ^ abcd[2] ^ abcd[3]
}

/// I
#[no_mangle]
pub extern "C" fn f3(abcd: &[u32]) -> u32 {
    abcd[2] ^ (abcd[1] | !abcd[3])
}

/// Rotate uint v left by amt bits
/// amt is signed integer so we can rotate right
#[no_mangle]
pub extern "C" fn rol(v: u32, amt: i16) -> u32 {
    if amt >= 0 {
        return v.rotate_left(amt as u32);
    }
    else {
        return v.rotate_right(-amt as u32);
    }
}

// pub extern "C" fn count_substrings(value: *const c_char, substr: *const c_char) -> i32 {
// let c_value = unsafe { CString::new(value, false) };
// let c_substr = unsafe { CString::new(substr, false) };
// match c_value.as_str() {
//     Some(value) => match c_substr.as_str() {
//         Some(substr) => value.match_indices(substr).count() as i32,
//         None => -1,
//     },
//     None => -1,
// }
// }
