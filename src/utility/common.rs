use std::{ffi::CString, os::raw::c_char};

pub fn c_char_to_str<'a>(char_vec: Vec<c_char>) -> String {
    let char_vec = char_vec.into_iter().map(|x| x as u8).collect();
    unsafe {
        let mut s = CString::from_vec_unchecked(char_vec)
            .into_string()
            .expect("Failed to convert layer name to string");
        s.retain(|c| c != '\0');
        s
    }
}
pub fn str_to_c_char<'a>(string: &'a str) -> *const c_char {
    CString::new(string).unwrap().as_ptr()
}
