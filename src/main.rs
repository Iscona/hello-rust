use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let serect_number:i16 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line !");

        println!("you guess the number is {}", guess);

        let guess:i16 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("输入格式有误！");
                continue;
            }
        };

        match guess.cmp(&serect_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }

    println!("geme over!!");
}
