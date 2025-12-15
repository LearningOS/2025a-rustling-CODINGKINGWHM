fn main() {
    let option = Some(12);
    let res = 42 + option; // 移除冗余的 unwrap_or(0)
    
    println!("{}", res);
}//
//