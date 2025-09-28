// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let answer = current_favorite_color(); // 调用函数获取当前最喜欢的颜色
    println!("My current favorite color is {}", answer); // 打印当前最喜欢的颜色
}

fn current_favorite_color() -> String { // 定义函数返回当前最喜欢的颜色
    String::from("blue") // 修改部分：将字符串字面量 "blue" 转换为 String 类型
}