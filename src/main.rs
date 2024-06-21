use std::io::{stdout,BufWriter};
use ferris_says::say;
fn main() {
    println!("Hello, world!");
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout().lock());
    say(&message,width,&mut writer).unwrap();
}