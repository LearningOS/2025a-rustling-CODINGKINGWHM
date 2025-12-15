#[cfg(test)]
mod tests {
    #[test]
    fn basic_assert() {
        // 验证简单逻辑
        let x = 5;
        assert!(x > 0, "x 应该是正数"); // 条件为真，测试通过
    }

    #[test]
    fn assert_equality() {
        // 更直观的相等性验证
        let s = String::from("rust");
        assert_eq!(s, "rust"); // 等价于 assert!(s == "rust")，但错误信息更友好
    }
}