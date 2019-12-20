use std::{ffi::CString, os::raw::c_char};
/*use std::collections::HashSet;
use std::iter::FromIterator;
use std::hash::Hash;*/

pub fn c_char_to_str(char_vec: Vec<c_char>) -> String {
    let char_vec = char_vec.into_iter().map(|x| x as u8).collect();
    unsafe {
        let mut s = CString::from_vec_unchecked(char_vec)
            .into_string()
            .expect("Failed to convert layer name to string");
        s.retain(|c| c != '\0');
        s
    }
}
/*pub fn remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T){
    let indexs_to_remove = {
        let mut indexs = vec![];
        let mut index = 0;
        for i in vec.iter(){
            if i == item{
                indexs.push(index);
            }
            index+=1;
        }
        indexs
    };
    for i in indexs_to_remove{
        vec.remove(i);
    }
}
pub fn set_from_vec<T: Eq + Hash>(vec: Vec<T>) -> HashSet<T> {
    HashSet::from_iter(vec)
}*/
