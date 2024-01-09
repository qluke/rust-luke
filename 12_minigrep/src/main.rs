use std::env;
use std::process;


// 将程序的标准输出重定向到文件 "output.txt" 中
// cargo run to poem.txt
// cargo run to poem.txt > output.txt

// 设置了一个环境变量 CASE_INSENSITIVE 的值为 "1"。这个环境变量在程序中被用来确定是否执行大小写敏感的搜索
// CASE_INSENSITIVE=1 cargo run to poem.txt
// CASE_INSENSITIVE=1 cargo run to poem.txt > output.txt

fn main() {
    // 从命令行参数中收集输入参数
    let args: Vec<String> = env::args().collect();
    
    // 创建Config对象，解析输入参数
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 运行搜索并处理错误
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

