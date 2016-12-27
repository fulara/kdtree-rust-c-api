extern crate libc;

#[no_mangle]
pub extern "C" fn count_substrings() -> i32 {
    3
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
