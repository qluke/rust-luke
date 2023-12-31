fn main() {
    // 示例 8-1：新建一个空的 vector 来储存 i32 类型的值
    let v: Vec<i32> = Vec::new();

    // 示例 8-2：新建一个包含初值的 vector
    let v = vec![1, 2, 3];

    // 示例 8-3：使用 push 方法向 vector 增加值
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 示例 8-5：使用索引语法或 get 方法来访问 vector 中的项
    let third: i32 = v[2];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // 示例 8-6：尝试访问一个包含 5 个元素的 vector 的索引 100 处的元素
    // 第一个 [] 方法，当引用一个不存在的元素时 Rust 会造成 panic
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // 示例 8-7：在拥有 vector 中项的引用的同时向其增加一个元素
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // immutable borrow later used here 不能在相同作用域中同时存在可变和不可变引用
    println!("The first element is: {}", first);

    // 示例 8-8：通过 for 循环遍历 vector 的元素并打印
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 示例 8-9：遍历 vector 中元素的可变引用
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // 示例 8-10：定义一个枚举，以便能在 vector 中存放不同类型的数据
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 示例 8-11：新建一个空的 String
    let mut s = String::new();

    // 示例 8-12：使用 to_string 方法从字符串字面量创建 String
    let s = "inital contents".to_string();

    // 示例 8-13：使用 String::from 函数从字符串字面量创建 String
    let s = String::from("initial contents");
    
    // 示例 8-14：在字符串中储存不同语言的问候语
    let hello = String::from("Hello");
    let hello = String::from("你好");

    // 示例 8-15：使用 push_str 方法向 String 附加字符串 slice
    let mut s = String::from("foo").push_str("bar");
 
    // 示例 8-16：将字符串 slice 的内容附加到 String 后使用它
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // 示例 8-17：使用 push 将一个字符加入 String 值中
    let mut s = String::from("lo");
    s.push('l');
    // s.push('llll');
    
    // 示例 8-18：使用 + 运算符将两个 String 值合并到一个新的 String 值中
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // let s4 = s1 + s2;
    // let s4 = s1 + &s2;
    // s1 在相加后不再有效的原因，和使用 s2 的引用的原因，与使用 + 运算符时调用的函数签名有关。
    // + 运算符使用了 add 函数，这个函数签名看起来像这样：fn add(self, s: &str) -> String {}
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 示例 8-19：尝试对字符串使用索引语法 Indexing into a string
    let s1 = String::from("hello");
    // let h = s1[0]; // `String` cannot be indexed by `{integer}`
    
    // use [] with a range to create a string slice containing particular bytes
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s);

    let hello = String::from("你好啊");
    let s = &hello[0..6];
    println!("s is {}", s);
    
    // 遍历字符串
    for c in "你好啊".chars() {
        println!("{}", c);
    }

    // 示例 8-20：新建一个哈希 map 并插入一些键值对
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 示例 8-21：用队伍列表和分数列表创建哈希 map
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 示例 8-22：展示一旦键值对被插入后就为哈希 map 所拥有
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // println!("field_name is {}", field_name);

    // 示例 8-23：访问哈希 map 中储存的蓝队分数
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue"));
    println!("score is {:?}", &score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 示例 8-24：替换以特定键储存的值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    println!("=========");

    // None的类型是Option<T>，而不是i32
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), Some(10));
    scores.insert(String::from("Blue"), None);
    println!("{:?}", scores);
    println!("=========");

    // 示例 8-25：使用 entry 方法只在键没有对应一个值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 示例 8-26：通过哈希 map 储存单词和计数来统计出现次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
// 示例 8-4：展示 vector 和其元素于何处被丢弃
// <- 这里 v 离开作用域并被丢弃

