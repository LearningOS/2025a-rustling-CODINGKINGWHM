// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 }, // 使用字段名
    Echo(String),
    ChangeColor(i32, i32, i32), // 增加第三个字段
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 }, // 使用字段名
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255), // 三个字段
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
