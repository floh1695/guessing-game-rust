use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
  guess_the_number();
}

fn guess_the_number() {
  println!("Welcome to the number guessing game!");

  let mut games = 0;
  let mut guesses = 0;
  loop {
    games += 1;

    println!("Starting game #{}", games);
    guesses += start_game();

    println!("Total guesses: {}", guesses);
  }
}

fn start_game() -> u32 {
  let secret_number = rand::thread_rng()
    .gen_range(1, 101);

  let mut tries = 0;
  loop {
    tries += 1;

    println!("Please take a guess:");

    let guess = take_guess();

    match guess.cmp(&secret_number) {
      Ordering::Less =>
        println!("Too low!"),
      Ordering::Greater =>
        println!("Too high!"),
      Ordering::Equal => {
        println!("You got it!");
        return tries;
      },
    }
  }
}

fn take_guess() -> u32 {
  let mut guess = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please type a number!");

  guess
}
