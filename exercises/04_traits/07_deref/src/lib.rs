// TODO: whenever `title` and `description` are returned via their accessor methods, they
//   should be normalized—i.e. leading and trailing whitespace should be removed.
//   There is a method in Rust's standard library that can help with this, but you won't
//   find it in the documentation for `String`.
//   Can you figure out where it is defined and how to use it?

// TODO：每当通过其访问器方法返回 `title` 和 `description` 时，它们
// 都应该被规范化 - 即应该删除前导和尾随空格。
// Rust 的标准库中有一种方法可以帮助解决这个问题，但你不会
// 在 `String` 的文档中找到它。
// 你能弄清楚它在哪里定义以及如何使用它吗？
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        &self.title.trim()
    }

    pub fn description(&self) -> &str {
        &self.description.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
