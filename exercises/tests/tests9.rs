extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // 实现核心函数，暴露到父模块
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    // 别名函数直接调用核心函数
    pub fn my_demo_function_alias(a: u32) -> u32 {
        super::my_demo_function(a)
    }
}

// 关键：将 Foo 模块的函数重新导出到 crate 根，让 extern 块能解析符号
pub use Foo::my_demo_function;
pub use Foo::my_demo_function_alias;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}