#![feature(int_log)]
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    let mut rng = rand::thread_rng();
    let mut still_guessing = true;
    let limit = set_limit();
    let limit_goal = 1+(limit.log2()/2_i32.log2()) ;
    loop{

        let mut won = false;
        let mut guess_count = 1;
        let my_guess = rng.gen_range(1..limit);

        println!("I'm thinking of a number between 1 and {}",limit);
        println!("Now you try to guess what it is.");

        while still_guessing {
            let inp = get_input()
                .trim()
                .parse::<i64>().unwrap();
            println!("\n\n\n");
            if inp < my_guess {
                println!("Too low. Try a bigger answer");
                guess_count+=1;
            }
            else if inp > my_guess {
                println!("Too high. Try a smaller answer");
                guess_count+=1;
            }
            else {
                println!("That's it! You got it in {} tries", guess_count);
                won = true;
                still_guessing = false;
            }
        }
        if won {
            match guess_count.cmp(&limit_goal) {
                Ordering::Less => println!("Very good."),
                Ordering::Equal => println!("Good."),
                Ordering::Greater => println!("You should have been able to get it in only {}", limit_goal),
            }

            println!("\n\n\n");
            still_guessing = true;
        } else {
            println!("\n\n\n");
        }
    }
}

fn set_limit() -> i64 {

    println!("                   Guess");
    println!("\n\n\n");
    println!("This is a number guessing game. I'll think");
    println!("of a number between 1 and any limit you want.\n");
    println!("Then you have to guess what it is\n");
    println!("What limit do you want?");

    let inp = get_input().trim().parse::<i64>().unwrap();

    if inp >= 2 {
        inp
    } 
    else {
        set_limit()
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Your input is not correct");
    input
}
