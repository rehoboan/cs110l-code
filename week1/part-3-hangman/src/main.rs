// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let guess_len = secret_word_chars.len();
    let mut guessed: Vec<char> = vec!['-';guess_len];
    let mut you_guessed: Vec<char> = Vec::new();
    let mut sum_guess = NUM_INCORRECT_GUESSES;

    println!("Welcome to Hangman!");


    while sum_guess > 0 {
        let s:String = guessed.iter().collect();
        println!("The word so far is {}",s);
        let you_guessed_str:String = you_guessed.iter().collect();
        println!("You have guessed the following letters:{}",you_guessed_str);
        println!("You have {} guesses left",NUM_INCORRECT_GUESSES);
        println!("Please guess a letter:");
        io::stdout()
            .flush()
            .expect("Error flushing stdout");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");
        let guess_vec:Vec<char> = guess.chars().collect();
        if secret_word_chars.contains(&guess_vec[0]){
            let index = guessed.iter().position(|&r|r==guess_vec[0]).unwrap();
            println!("{}",index);
        }
        else{
            sum_guess -= 1;
        }
    }

    }



