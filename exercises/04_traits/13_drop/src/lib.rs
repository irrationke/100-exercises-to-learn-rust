// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.
// TODO：实现所谓的“投掷炸弹”：一种在被投掷时会引发恐慌的类型
//  除非对其执行了某项操作。
//  您可以在下面的测试中看到预期的 API。

pub struct DropBomb {
    defuse: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defuse: false }
    }

    pub fn defuse(&mut self) {
        self.defuse = true;
    }

}

impl Drop for DropBomb {
    fn drop(&mut self) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
        // 炸弹掉落时应发生恐慌
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
        // 炸弹掉落时不应惊慌失措
        // 因为它已被拆除
    }
}
