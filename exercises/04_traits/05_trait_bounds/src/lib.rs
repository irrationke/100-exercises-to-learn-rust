// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

/// Return the minimum of two values.

// TODO：将必要的特征边界添加到 `min`，以便成功编译。
// 有关您可能需要的特征的更多信息，请参阅 `std::cmp` 模块的文档。
//
// 注意：有不同的特征边界可以让编译器满意，但它们带有
// 不同的_语义_。我们将在课程后面讨论有序
// 集合（例如 BTreeMap）时介绍这些差异。

/// 返回两个值中的最小值。

pub fn min<T:Eq + PartialEq + PartialOrd>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
