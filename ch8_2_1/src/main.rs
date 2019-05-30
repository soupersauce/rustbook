// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     // s1.push_str(s2);
//     // println!("s2 is {}", s2);

//     let mut s = String::from("lo");
//     s.push('l');
//     println!("s is {}", s);

//     let s3 = s1 + &s2;
//     println!("{}", s3);
// }
// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{}-{}-{}", s1, s2, s3);
//     println!("{}", s)
// }
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    print!("{}", s);

    for c in "नमस्ते".chars() {
            println!("{}", c);
            
    }
    for b in "नमस्ते".bytes() {
            println!("{}", b);
            
    }
}

