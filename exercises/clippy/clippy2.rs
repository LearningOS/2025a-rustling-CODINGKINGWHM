fn main() {
    let mut res = 42;
    let option = Some(12);

    // 替代 match，简洁且无冗余
    if let Some(val) = option {
        res += val;
    }
}