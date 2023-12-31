fn main() {
    // 1、定义枚举
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // 2、仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
    
    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));

    // 3、使用 impl 在枚举上定义方法
    let m = Message::Write(String::from("hello"));
    m.call();

    // 4、Option 枚举定义于标准库中
    // enum Option<T> {
    // Some(T),
    // None,
    // }    
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    // 5、match 模式匹配
    value_in_cents(Coin::Penny);

    // 6、match 绑定匹配的模式的部分值
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 7、通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 匹配任意值而不绑定到该值
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    // 8、if let 失去 match 强制要求的穷尽性检查 
    // match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }else {
        println!("other");
    }
}
enum IpAddrKind {
    V4,
    V6,
}
enum IpAddr {
    V4(String),
    V6(String),
}
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}