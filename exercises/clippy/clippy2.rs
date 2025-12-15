// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 优化点：用 if let 替代 for 循环处理 Option，更语义化
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}
