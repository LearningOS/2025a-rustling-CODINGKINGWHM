// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

//

// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}

fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // "blue" 是 &str 类型，调用 string_slice
    string("red".to_string()); // "red".to_string() 是 String 类型，调用 string
    string(String::from("hi")); // String::from("hi") 是 String 类型，调用 string
    string("rust is fun!".to_owned()); // "rust is fun!".to_owned() 是 String 类型，调用 string
    string("nice weather".into()); // "nice weather".into() 是 String 类型，调用 string
    string(format!("Interpolation {}", "Station")); // format! 宏返回 String 类型，调用 string
    string_slice(&String::from("abc")[0..1]); // &String::from("abc")[0..1] 是 &str 类型，调用 string_slice
    string_slice("  hello there ".trim()); // "  hello there ".trim() 是 &str 类型，调用 string_slice
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // "Happy Monday!".to_string().replace("Mon", "Tues") 是 String 类型，调用 string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // "mY sHiFt KeY iS sTiCkY".to_lowercase() 是 String 类型，调用 string
}