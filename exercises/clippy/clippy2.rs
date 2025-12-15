fn main() {
    let option = Some(12);
    let res = 42 + option.unwrap_or_default(); 
    
    println!("{}", res);
}