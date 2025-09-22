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
String ("blue");
    String_slice("red".to_string());
    String (String::from("hi"));
    String_slice("rust is fun!".to_owned());
    String_slice("nice weather".into());
    String(format!("Interpolation {}", "Station"));
    String_slice(&String::from("abc")[0..1]);
    String_slice("  hello there ".trim());
    String("Happy Monday!".to_string().replace("Mon", "Tues"));
    String_slice("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
