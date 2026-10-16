use std::ffi::c_int;

extern "C" {
    fn abs(n: c_int) -> c_int;
}

pub fn safe_abs(n: i32) -> Option<u32> {
    if n == i32::MIN {
        None
    } else {
        let res = unsafe { abs(n) };
        Some(res as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(safe_abs(-42), Some(42));
        assert_eq!(safe_abs(100), Some(100));
        assert_eq!(safe_abs(i32::MIN), None);
    }
}
