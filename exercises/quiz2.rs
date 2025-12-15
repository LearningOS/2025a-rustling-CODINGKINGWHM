pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 补全函数签名：输入是 (String, Command) 的 Vec，输出是 String 的 Vec
    pub fn transformer(input: &Vec<(String, Command)>) -> Vec<String> {
        // 声明输出向量，类型为 Vec<String>
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // 匹配命令并执行对应操作
            let result = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => {
                    let mut s = string.clone();
                    s.push_str(&"bar".repeat(*n));
                    s
                }
            };
            output.push(result);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入 my_module 中的 transformer 函数
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}