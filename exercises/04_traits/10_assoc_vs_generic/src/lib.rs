// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// TODO：定义一个新的特征“Power”，它有一个方法“power”，可以将“self”提升到“n”的幂。
// 特征定义及其实现应该足以让测试编译并通过。
//
// 建议：您可能想编写一个通用实现来同时处理所有情况。但是，这相当复杂，需要使用额外的包（即“num-traits”）。
// 即便如此，最好还是使用一个简单的宏来避免高度通用实现的复杂性。如果您有兴趣了解更多信息，请查看“Rust 宏小册子”（https://veykril.github.io/tlborm/）。
// 不过你不必这样做：手动编写三个单独的实现是完全可以的。只有你好奇的时候才可以进一步探索。

use std::u32;

pub trait Power<Exponent = Self> {
    type Output;

    fn power(&self, n: Exponent) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;

    fn power(&self, n: u16) -> Self::Output {
        self.pow(n.into())
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(&self, n: &u32) -> Self::Output {
        self.power(*n)
    }
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(&self, n: u32) -> Self::Output {
        self.pow(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
