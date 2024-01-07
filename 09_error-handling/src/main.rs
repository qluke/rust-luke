use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs::File;
use std::fs;

fn main() {
    // 示例 9-1：尝试访问超越 vector 结尾的元素，这会造成 panic!
    // Unwinding the Stack or Aborting in Response to a Panic
    // panic!("crash and burn");

    // 示例 9-2：当设置 RUST_BACKTRACE 环境变量时 panic! 调用所生成的 backtrace 信息
    // $ RUST_BACKTRACE=1 cargo run
    let v = vec![1, 2, 3];
    // v[99];
    
    // 6: error_handling::main
    // at ./src/main.rs:7:6

    // 示例 9-3：打开文件
    let f = File::open("hello.txt");
    
    // 示例 9-4：使用 match 表达式处理可能会返回的 Result 成员
    let f = File::open("hello.txt");

    // File::open 函数的返回值类型是Result<T, E>: std::result::Result<std::fs::File, std::io::Error>
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    // 示例 9-5：使用不同的方式处理不同类型的错误
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    
    // 这段代码有着如示例 9-5 一样的行为，但没有包含任何 match 表达式且更容易阅读
    fn main() {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    // 失败时 panic 的简写：unwrap 和 expect
    // unwrap：失败时调用 panic! 
    // 所有的 unwrap 调用都打印相同的信息
    let f = File::open("hello.txt").unwrap();

    // expect：允许我们选择 panic! 的错误信息
    // 会更容易找到代码中的错误信息来自何处
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    
    
    
}
// 示例 9-6：一个函数使用 match 将错误返回给代码调用者
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 示例 9-7：一个使用 ? 运算符向调用者返回错误的函数
// 只能在返回 Result 或者其它实现了 std::ops::Try 的类型的函数中使用 ? 运算符
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 示例 9-8：问号运算符之后的链式方法调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 示例 9-9: 使用 fs::read_to_string
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
