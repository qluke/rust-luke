/** 
所有权规则：
Rust 中的每一个值都有一个被称为其所有者（owner）的变量。
值在任一时刻有且只有一个所有者。
当所有者（变量）离开作用域，这个值将被丢弃。
 */
fn main() {
    let mut s = String::from("hello"); // 从此处起，s 开始有效
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`

    // 1、String 由三部分组成：一个指向存放字符串内容内存的指针，一个长度，和一个容量。
    let s1 = String::from("hello");
    // s1 的拷贝并绑定到 s2 上
    // Rust 同时使第一个变量无效了，这个操作被称为 移动（move），而不是浅拷贝
    let s2 = s1;
    // println!("{}, world!", s1);

    // 2、深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 3、刚刚的情况只作用于堆，而不会发生在栈上的数据
    // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 4、所有权与函数 
    // 将值传递给函数在语义上与给变量赋值相似
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ... 
    // println!("{}", s);           // ... 所以到这里不再有效

    // 5、返回值与作用域 
    // 返回值也可以转移所有权
    let s1 = gives_ownership();         // gives_ownership 将返回值
 
    // 6、引用与借用 
    // calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权，在调用 calculate_length 后仍能使用该 String
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    // 7、引用 & 默认是不可变的，我们无法通过引用修改内容
    // 而可变引用 &mut 有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用
    // 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
    let mut s1 = String::from("hello");
    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。它们的作用域没有重叠
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{}", r3);

    // 8、切片 Slice 类型
    let s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5
    println!("first_word: {}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    // 对部分 String 的引用，world 将是一个包含指向 s 索引 6 的指针和长度值 5 的 slice。
    let world = &s[6..11];
    println!("world: {}", world);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// 此作用域已结束，s 不再有效


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
  
fn gives_ownership() -> String {           // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string                              // 返回 some_string 并移出给调用的函数
}
