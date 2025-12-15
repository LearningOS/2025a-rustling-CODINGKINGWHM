// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // 合并变量初始化与 Option 取值逻辑，更简洁
    let res = 42 + option.unwrap_or(0);
    let option = Some(12);
    
    println!("{}", res);
}