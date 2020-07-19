fn main() {
    let mut s1 = String::from("hello");

    let _r1 = &mut s1;

    {
        let _r2 = &mut s1;
    }
    // let len = calcuate_length(&mut s1);
    println!("{}，{}", _r1, _r1);
}

// fn calcuate_length(s: &mut String) -> usize {
//     s.push_str(", world!");
//     s.len()
// }