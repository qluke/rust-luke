// 示例 10-4：两个函数，不同点只是名称和签名类型
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 一个使用泛型参数的 largest 函数定义
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}
// 示例 10-15：一个可以用于任何实现了 PartialOrd 和 Copy trait 的泛型的 largest 函数
fn largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// 示例 10-8：使用两个泛型的 Point，这样 x 和 y 可能是不同类型
struct Point<T, U> {
    x: T,
    y: U,
}
// 示例 10-9：在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
struct Point2<T> {
    x: T,
    y: T,
}
// 示例 10-11：方法使用了与结构体定义中不同类型的泛型
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 示例 10-10：构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 孤儿规则（orphan rule）：只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article: NewsArticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());    
    // 默认实现  
    println!("New article available! {}", article.summarize2());    

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // 调用默认方法
    println!("1 new tweet: {}", tweet.summarize3());
}

// 类似 Java 里的接口
pub trait Summary {
    // 示例 10-12：Summary trait 定义，它包含由 summarize 方法提供的行为
    fn summarize(&self) -> String;
    // 示例 10-14：Summary trait 的定义，带有一个 summarize 方法的默认实现
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
    fn summarize_author(&self) -> String;
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// 示例 10-13：在 NewsArticle 和 Tweet 类型上实现 Summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// 以下等同, trait 作为参数
pub fn notify(item: impl Summary) {}
pub fn notify2<T: Summary>(item: T) {}

// 以下等同
pub fn notify3(item1: impl Summary, item2: impl Summary) {}
pub fn notify4<T: Summary>(item1: T, item2: T) {}

// 以下等同
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// fn some_function<T, U>(t: T, u: U) -> i32 where T: Display + Clone,    U: Clone + Debug {}

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
