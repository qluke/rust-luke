// cargo test 命令会运行项目中所有的测试
#[cfg(test)]
mod tests {
    use super::*;
    // 1、使用 #[test] 属性标明哪些函数是测试
    #[test]
    fn eq_it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // 2、测试 assert_ne
    #[test]
    fn not_eq_it_works() {
        assert_ne!(2 + 2, 5);
    }
    // 3、因调用了 panic! 而失败的测试
    #[test]
    fn another() {
        // panic!("Make this test fail");
    }
    // 4、使用 assert! 宏来检查结果
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }
    // 5、自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));

        // let result = greeting("Car");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
    // 6、使用 should_panic 检查 panic，带有特定错误信息的 panic!
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 7、将 Result<T, E> 用于测试
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    // 8、不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 --test-threads 参数
    // cargo test -- --test-threads=1

    // 显示函数输出
    // cargo test -- --show-output

    // 通过指定名字来运行部分测试（单个、多个）
    // cargo test not_eq_it_works
    // cargo test it_works
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess {
            value
        }
    }
}
