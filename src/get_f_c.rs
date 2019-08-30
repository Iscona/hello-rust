use std::io;

fn main() {

    println!("请选择您要查询的类型：(默认[1])\n[1]摄氏度---->>华氏度\n[2]华氏度---->>摄氏度");

    let trans_type = get_trans_type();

    println!("您要查询的类型是{}，请输入：", trans_type);
    

    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("获取失败");

    let num: f64 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => 0.0
    };

    let ret: f64;
    if trans_type == 1 {
        ret = c_to_f(num);
    } else {
        ret = f_to_c(num);
    }


    println!("结果是{}", ret);
}

fn get_trans_type() -> i32 {
    let mut trans_type = String::new();

    io::stdin().read_line(&mut trans_type)
        .expect("failed!");

    let mut trans_type: i32 = match trans_type.trim().parse(){
        Ok(t) => t,
        Err(_) => 1
    };

    trans_type = if trans_type == 1 {
        1
    } else if trans_type == 2 {
        2
    } else {
        1
    };

    // trans_type = for i in 1..3 {
    //     if i == trans_type {
    //         break i;
    //     } else {
    //         break 1;
    //     }
    // };

    trans_type
}

fn c_to_f(c: f64) -> f64 {
    (9.0 / 5.0) * c + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (5.0/9.0) * (f - 32.0)
}