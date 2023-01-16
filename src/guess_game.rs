use std::io;

pub struct Guess {
    pub random_number: i32
}

impl Guess{
    pub fn GuessGame(&self){
        loop {
            let mut buffer = String::new();
            println!("Write your guess: ");
            io::stdin().read_line(&mut buffer);
            let user_guess: i32 = buffer.trim().parse().unwrap();
            if user_guess == self.random_number{
                println!("You win");
                break;
            }else if user_guess > self.random_number{
                println!("pick a smaller number");
            }else{
                println!("pick a greater number");
            }
        }
    }
}