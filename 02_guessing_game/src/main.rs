use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutability 代表可变

        // 将用户在标准输入中输入的任何内容都追加到一个字符串中（而不会覆盖其内容），
        // 所以它需要字符串作为参数。这个字符串应是可变的，以便该方法可以更改其内容。
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");  //this `Result` may be an `Err` variant, which should be handled

        // expect 调用换成 match 语句，从而实现遇到错误就崩溃转换成处理错误
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
