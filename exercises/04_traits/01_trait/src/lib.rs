// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

// 定义一个名为“IsEven”的特征，它有一个方法“is_even”，如果“self”为
// 偶数，则返回“true”，否则返回“false”。
//
// 然后为“u32”和“i32”实现该特征。

/// Define a trait.
trait IsEven {
    fn is_even(self) -> bool;
}

/// implementing IsEven for u32.
impl IsEven for u32 {
    fn is_even(self) -> bool {
        if self % 2 == 0 {
            true
        } else {
            false
        }
    }
}

/// implementing IsEven for i32.
impl IsEven for i32 {
    fn is_even(self) -> bool {
        if let 0 = self % 2 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
