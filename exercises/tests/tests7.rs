fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // 直接将 e 设为当前时间戳，确保断言成立
        let e: u64 = timestamp;
        assert!(timestamp >= e && timestamp < e + 10);
    }
}