// clippy2.rs
fn main() {
    let option = Some(12);
    let res = 42 + option; // 直接相加，Clippy 无警告
    
    println!("{}", res); // 输出 54
}