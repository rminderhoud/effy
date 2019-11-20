use std::convert::From;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub type string_t = FfiString;

#[repr(C)]
pub struct FfiString {
    chars: *mut c_char,
}

impl FfiString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(ptr: *mut Self) -> Self {
        unsafe { Self::from(Box::from_raw(ptr).chars as *const c_char) }
    }

    pub fn set_string(&mut self, s: &str) {
        unsafe {
            CString::from_raw(self.chars);
        }

        let new_str = CString::new(s.as_bytes()).unwrap_or_default();
        self.chars = CString::from(new_str).into_raw();
    }

    pub fn set_c_string(&mut self, s: *const c_char) {
        let raw_str = unsafe { CStr::from_ptr(s) };
        self.set_string(&raw_str.to_string_lossy());
    }

    pub fn to_string(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.chars) };
        s.to_string_lossy().to_string()
    }
}

impl Default for FfiString {
    fn default() -> Self {
        FfiString {
            chars: CString::default().into_raw(),
        }
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.chars);
        }
    }
}

impl From<&str> for FfiString {
    fn from(s: &str) -> Self {
        let mut res = Self::new();
        res.set_string(s);
        res
    }
}

impl From<*const c_char> for FfiString {
    fn from(s: *const c_char) -> Self {
        let mut res = Self::new();
        res.set_c_string(s);
        res
    }
}

#[no_mangle]
pub unsafe extern "C" fn string_new() -> *mut string_t {
    FfiString::new().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn string_free(ptr: *mut string_t) {
    FfiString::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn string_update(ptr: *mut string_t, new_str: *const c_char) {
    let mut s = FfiString::from_raw(ptr);
    s.set_c_string(new_str);
    s.into_raw();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unsafe {
            let s = string_new();
            assert_eq!((*s).to_string(), "");

            let new_string = CString::new("test").unwrap_or_default();
            let new_string_raw = new_string.into_raw();
            string_update(s, new_string_raw);
            assert_eq!((*s).to_string(), "test");

            string_free(s);
        }
    }
}
