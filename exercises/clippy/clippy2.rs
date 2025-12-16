fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = option { // 替换 for 循环
        res += x;
    }

    println!("{res}");
}