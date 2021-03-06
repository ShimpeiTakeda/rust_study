extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数当てゲームをしよう!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("予想する数を入れてね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed o read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("予想は: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
