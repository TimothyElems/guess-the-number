use std::io::stdin; // This is a comment

use rand::Rng;

fn main() {
    '_outer_loop: loop {
        // Target number
        // let _number: u32 = 10;
        let _number: i32 = rand::thread_rng().gen_range(1..160);

        // Prompt
        println!("Guess a number >>>");

        loop {
            // Empty new string
            let mut line = String::new();

            // Reading the input
            let _input = stdin().read_line(&mut line);

            let _guess: Option<i32> = _input.ok().map_or(None, |_| line.trim().parse().ok());

            match _guess {
                None => println!("enter a number... "),
                Some(n) if n == _number => {
                    println!("Bravo! You got it!");
                    break '_outer_loop;
                }
                Some(n) if n < _number => println!("Too low"),
                Some(n) if n > _number => println!("Too high"),
                Some(_) => println!("Error"),
            }
        }
    }
}
