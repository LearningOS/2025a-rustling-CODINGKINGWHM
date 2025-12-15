pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
        assert!(is_even(4)); // 补充更多用例
    }

    // 新增测试：奇数返回 false
    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5)); // 断言 5 是奇数，返回 false
        assert!(!is_even(7)); // 补充更多用例
    }
}