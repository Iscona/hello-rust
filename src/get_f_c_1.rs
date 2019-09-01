use std::io;

fn main() {
    loop {
        println!("==================================");
        println!("1. 华氏度/摄氏度转换");
        println!("2.  n 阶斐波那契数列");
        println!("3. The Twelve Days of Christmas");
        println!("输入功能数，或者其他任意数字退出。");
        println!("==================================");

        let mut func_num = String::new();

        io::stdin()
            .read_line(&mut func_num)
            .expect("error");

        let func_num: i32 = match func_num.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        match func_num {
            1 => f_c_converter(),
            2 => fibon(),
            3 => christmas(),
            _ => {
                println!("滚吧！");
                break;
            }
        }
    }
}

fn christmas() {
    println!("The Twelve days of Christmas. ");
    let numth_arr = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let niao_arr = [
        "And a partridge in a pear tree.",
        "Two turtle doves, ",
        "Three French hens, ",
        "Four calling birds, ",
        "Five golden rings, ",
        "Six geese a-laying, ",
        "Seven swans a-swimming, ",
        "Eight maids a-milking, ",
        "Nine ladies dancing, ",
        "Ten lords a-leaping, ",
        "Eleven pipers piping, ",
        "Twelve drummers drumming, ",
    ];

    for _i in 0..numth_arr.len(){
        println!("On the {} day of Christmas, my true love sent to me: ", numth_arr[_i]);

        //for 循环
        // for _j in (0.._i+1).rev() {

        //     if _i == 0 {
        //         println!("A partridge in a pear tree.");
        //         continue;
        //     }
        //     println!("{}", niao_arr[_j]);
        // }

        //while循环
        let mut n = _i + 1;
        while n > 0 {
            if _i == 0 {
               println!("A partridge in a pear tree."); 
            } else {
                println!("{}", niao_arr[n-1]);
            }
            
            n -= 1;
        }
    }
}
// fn get_niao(numth: usize) -> String {


//     "11111111".to_string()
// }
// fn fibon() {
//     println!("输入n阶");
//     let mut n = String::new();
//     io::stdin().read_line(&mut n).expect("error");
//     let n: u32 = n.trim().parse().expect("error");
//     for i in 1..n + 1 {
//         print!("{}\t", calc_fibon(i));
//         if i % 5 == 0 {
//             println!("");
//         }
//     }
//     println!("");
// }

// fn calc_fibon(n: u32) -> u32 {
//     if n <= 2 {
//         1
//     } else {
//         calc_fibon(n - 1) + calc_fibon(n - 2)
//     }
// }
fn fibon() {
    println!("输入一个值，n阶");

    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("error");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 1
    };

    for _i in 1..num + 1 {
        print!("{}\t", getfeibo(_i));
    }

    println!("");
}

fn getfeibo(num: u32) -> u32 {
    if num <= 2 {
        1
    } else {
        getfeibo(num - 1) + getfeibo(num - 2)
    }
}

fn f_c_converter() {
    println!(
        "输入一个值，以F结尾代表华氏度，或者以C结尾为摄氏度。比如77F。"
    );

    loop {
        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("error");

        input_str = input_str.trim().to_string();

        let len = input_str.len() - 1;

        let (number, unit) = input_str.split_at(len);

        let number: f64 = number.parse().expect("error");

        //???? &unit.to_uppercase()[..]
        match unit {
            "F" => {
                let c = f_to_c(number);
                println!("华氏度 {} 等同于摄氏度 {}", number, c);
            },
            "C" => {
                let f = c_to_f(number);
                println!("摄氏度 {} 等同于华氏度 {}", number, f);
            }
            _ => {
                println!(
                    "输入一个值，以F结尾代表华氏度，或者以C结尾为摄氏度。比如77F。"
                );
                continue;
            }
        }
        break;
    }
}

fn f_to_c(number: f64) -> f64 {
    (5.0/9.0) * (number-32.0) 
}

fn c_to_f(number: f64) -> f64 {
    (9.0/5.0) * number + 32.0 
}