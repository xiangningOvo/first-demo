use std::io;
use std::io::{stdout, BufWriter};
use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout().lock());
    say(&message,width,&mut writer).unwrap();

    let rand = rand::thread_rng().gen_range(1..=100);
    // println!("随机数字为{rand}");

    loop {
        println!("猜测一下");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜想的数字是{guess_int}");

        match guess_int.cmp(&rand) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("恭喜猜对了🎉");
                break;
            }
        }

    }

}