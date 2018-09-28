use std::ffi::CStr;

pub fn c_to_r_array(mut external_pointer: *const *const i8) -> Result<Vec<String>, String> {
    let mut array: Vec<String> = vec![];
    if !external_pointer.is_null() {
        loop {
            unsafe {
                let internal_pointer = *external_pointer;
                if (internal_pointer).is_null() {
                    break;
                }
                array.push(
                    CStr::from_ptr(internal_pointer)
                        .to_string_lossy()
                        .into_owned(),
                );
                external_pointer = external_pointer.add(1)
            };
        }
        Ok(array)
    } else {
        Err("".to_string())
    }
}
