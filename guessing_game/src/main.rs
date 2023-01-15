use std::io;  // prelude
// 使用 io 输入/输出库 来至 std 标准库里
use rand::Rng;  // trait
use std::cmp::Ordering;
fn main() {
    println!("Guess the number！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        //变量一般默认不可修改，mut修饰为可变变量

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        println!("You guessed: {guess}");

        // shadow
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    
}
