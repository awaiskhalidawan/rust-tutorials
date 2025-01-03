use std::ffi::CString;

fn main() {
    // Create a CString from a string and find the length of the string.
    let c_string = CString::new("Hello, world!").unwrap();
    let ptr = c_string.as_ptr();
        
    unsafe {
        println!("CString value: {:?}", std::ffi::CStr::from_ptr(ptr));
        let len = libc::strlen(ptr);
        println!("CString length: {:?}", len);
        println!("CString location in memory: {:?}", ptr);        
    }

    // Create another CString from a string and try to find the substring in the first CString.
    let c_string_2 = CString::new("world!").unwrap();
    let ptr_2 = c_string_2.as_ptr();

    unsafe {
        let ptr_3 = libc::strstr(ptr, ptr_2);
        if !ptr_3.is_null() {
            println!("Found substring: {:?}", std::ffi::CStr::from_ptr(ptr_3));
        }
    }
}
