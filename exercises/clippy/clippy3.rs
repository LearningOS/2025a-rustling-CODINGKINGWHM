// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

fn main() {
    let my_option: Option<()> = None;
    // 移除无意义的 val 占位，直接处理 Option 空值场景（Clippy 会提示 unused_variable）
    if my_option.is_none() {
        // 显式处理 None 场景，比空分支更语义化
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // 直接创建空 Vec，替代 resize 清空（Clippy 提示 unnecessary_resize）
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 保留 std::mem::swap（Clippy 推荐的交换方式），补充注释增强可读性
    // 交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}