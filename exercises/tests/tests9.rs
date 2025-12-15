extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    // 为原函数添加别名映射
    #[link_name = "my_demo_function_alias"]
    #[no_mangle]
    pub extern "Rust" fn my_demo_function_alias(a: u32) -> u32 {
        super::my_demo_function(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}