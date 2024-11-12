// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.
// TODO：为 `WrappingU32` 类型实现 `From` 特征以使 `example` 能够编译。
pub struct WrappingU32 {
    value: u32,
}
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}
fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
