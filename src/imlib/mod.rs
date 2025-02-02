use std::ffi::CString;

use crate::{imlib_load_image, Imlib_Image};



pub unsafe fn load_image(path: &str) -> Imlib_Image {
    let image_path_c_str = CString::new(path).unwrap();
    imlib_load_image(image_path_c_str.as_ptr()) 
}