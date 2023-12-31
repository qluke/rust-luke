fn main() {
    // 1、定义并实例化结构体，创建 User 结构体的实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 2、改变 User 实例 email 字段的值
    user1.email = String::from("anotheremail@example.com");

    // 3、使用结构体更新语法为一个 User 实例设置一个新的 email 值，不过其余值来自 user1 变量中实例的字段
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    
    // 4、使用没有命名字段的元组结构体
    let black = Color(0, 0, 0);

    // 5、没有任何字段的类单元结构体
    let subject = AlwaysEqual12;

    // 6、在函数签名和调用的地方有 &， main 函数就可以保持 rect1 的所有权并继续使用它（ area 函数借用结构体而不是获取它的所有权）
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);

    // 7、dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    // 8、方法语法
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);
    dbg!(&sq);

}

impl Rectangle {
    // &self 实际上是 self: &Self 的缩写，表示我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 在结构体定义之前加上外部属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Color(i32, i32, i32);
struct AlwaysEqual12;


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}