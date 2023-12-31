// 1、模块树，通过执行 cargo new --lib restaurant，来创建一个新的名为 restaurant 的库
mod front_of_house {
    pub mod hosting {
        // 使用 pub 关键字暴露路径
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 2、调用一个函数，我们需要知道它的路径，路径有两种形式：绝对路径、相对路径
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 3、使用 super 起始的相对路径
fn serve_order() {}

mod back_of_house0 {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// 4、创建公有的结构体和枚举
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant1() {
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释，将会导致编译失败；我们不被允许
    // 看到或更改随餐搭配的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

// 5、使用 use 关键字将名称引入作用域
use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
// 6、使用 as 关键字提供新的名称
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
 
// pub use 重导出名称，这样做将项引入作用域并同时使其可供其他代码引入自己的作用域
// pub use crate::front_of_house::hosting;
// 嵌套路径来消除大量的 use 行，使用嵌套路径将相同的项在一行中引入作用域
// use std::{cmp::Ordering, io};
