use std::io::Write;

mod calc;

fn main() {
    println!("欢迎使用计算器!");
    println!("输入算术表达式, 按回车开始计算, 支持 + - * / () ^ 符号。");
    println!("输入退出请输入q。");

    loop {
        let mut input = String::new();
        // rust 行缓冲，需要刷新才能显示在输出通一行
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "q" {
            println!("Bye!");
            break;
        } else if input.trim() == "" {
            continue;
        }
        match calc::calc(&input) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Err: {}", err),
        }
    }
}
