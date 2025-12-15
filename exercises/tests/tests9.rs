// 外部块声明：使用 "C" ABI 避免 Rust 符号混淆，简化链接逻辑
extern "C" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // 导出原函数（C ABI + 无混淆）
    #[no_mangle]
    pub extern "C" fn my_demo_function(a: u32) -> u32 {
        a
    }

    // 别名直接转发到原函数（无需额外实现，仅映射符号）
    #[no_mangle]
    #[link_name = "my_demo_function"]
    pub extern "C" fn my_demo_function_alias(a: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            // 两个函数最终都调用 Foo::my_demo_function
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}