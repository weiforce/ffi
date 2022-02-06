extern crate libc;
extern crate url;

use std::ffi::{CStr,CString};
use url::{Url};

use std::{thread, time};

#[no_mangle]
pub extern "C" fn get_query (arg1: *const libc::c_char) -> *const libc::c_char {

    let s1 = unsafe { CStr::from_ptr(arg1) };

    let str1 = s1.to_str().unwrap();

let ten_millis = time::Duration::from_millis(10000);
let now = time::Instant::now();

thread::sleep(ten_millis);


    CString::new("test").unwrap().into_raw()
//    let parsed_url = Url::parse(
//        str1
//    ).unwrap();

//    CString::new(parsed_url.query().unwrap().as_bytes()).unwrap().into_raw()
}

