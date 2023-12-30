fn main() {
    // 在变量名前加上 mut 使得它们可变
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 常量声明
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // 第一个变量被第二个变量遮蔽（shadow）
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    // 第一个 spaces 变量是一个字符串类型，第二个 spaces 变量是一个数字类型
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);


    let num1: u8  = 255;
    let num2: u16  = 65535;
    let num3: i32  = -996;
    let num4: isize  = -996;
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of num1 is: {}", num1);
    println!("The value of num2 is: {}", num2);
    println!("The value of num3 is: {}", num3);
    println!("The value of num4 is: {}", num4);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);


    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("floored: {}", floored);
    println!("remainder: {}", remainder);

    let _t = true;
    let _f: bool = false; // with explicit type annotation
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // 元组类型，在小括号内写入以逗号分隔的值列表来创建一个元组
    let tup = (500, 6.4, 1);

    // 首先创建一个元组并将其绑定到变量 tup 上。 然后它借助 let 来使用一个模式匹配 tup，并将它分解成
    // 三个单独的变量 x、y 和 z。 这过程称为解构（destructuring），因为它将单个元组分为三部分。
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // 数组类型
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // 使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 变量名为 a 的数组将包含 5 个元素，这些元素的值初始化为 3。这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。
    let a = [3; 5];

    let first = a[0];
    let second = a[1];
    println!("first:  {} second: {}", first, second);

    // 函数调用
    another_function();
    another_function_2(5);
    print_labeled_measurement(5, 'h');

    // 语句和表达式
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    // if 表达式
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // 外层循环有一个标签 counting_up，它将从 0 数到 2。没有标签的内部循环从 10 向下数到 9。
    // 第一个没有指定标签的 break 将只退出内层循环。break 'counting_up; 语句将退出外层循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // 将操作的结果从循环中传递给其它的代码
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while 条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 使用 for 遍历集合
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
        

fn another_function() {
    println!("Another function.");
}
fn another_function_2(x: i32) {
    println!("Another function.The value of x is: {}", x);
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}