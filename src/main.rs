use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind};
use random_word::Lang;
use std::fs;
use std::io;

const MIN_WORD_LENGTH: usize = 4;
const MAX_WORD_LENGTH: usize = 6;

fn main() -> io::Result<()> {
    let mut num_guesses_left: i8 = 6;
    let mut guessed_letters = String::new();

    print_welcome()?;

    // generate a random word
    let word = generate_word().to_string();
    //let word = String::from("borox");
    println!("The word is {}", word);

    let mut answer = String::new();
    for _i in 0..word.len() {
        answer.push('_');
    }

    while num_guesses_left > 0 {
        print_rustman(num_guesses_left)?;
        print_answer(&answer)?;

        let letter: char = ask(&mut guessed_letters).unwrap();

        let hits: Vec<(usize, &str)> = check_guess(letter, word.as_str());
        if hits.is_empty() {
            num_guesses_left -= 1;
            continue;
        } else {
            answer = fill_answer(hits, &mut answer);
        }

        if word == answer {
            print_win(word);
            std::process::exit(0);
        }
    } // end while loop

    print_lose(word);
    Ok(())
} // fn main

/// prints the game's welcome message
fn print_welcome() -> io::Result<()> {
    //clearscreen::clear().unwrap();
    let welcome_msg: &str = "Welcome to Rustman!\n\n\
                                Rustman is like Hangman, but instead of saving a\
                                human from capital punishment\n\
                                you'll be saving the robot from rusting away!\n\n\
                                Press Enter to start!";
    println!("{}", welcome_msg);

    loop {
        let event = read()?;

        if event == Event::Key(KeyCode::Enter.into()) {
            break;
        }
    }

    println!("Starting game!");

    clearscreen::clear().unwrap();
    Ok(())
} // fn print_welcome

fn generate_word() -> &'static str {
    let mut word: &str = "";
    while word.len() < MIN_WORD_LENGTH || word.len() > MAX_WORD_LENGTH {
        word = random_word::gen(Lang::En);
    }

    word
}

/// prints a text picture version of rustman
/// that depends on how many guesses are left.
/// pictures are contained in files of the type <number>_guesses_left.txt
fn print_rustman(num_guesses_left: i8) -> io::Result<()> {
    clearscreen::clear().unwrap();

    // TODO: figure out install pathing
    match num_guesses_left {
        6 => {
            let contents = fs::read_to_string("res\\rustman6.txt")
                .expect("File rustman6.txt could not be opened!");
            println!("{}", contents);
            println!("6 guesses left");
        }
        5 => {
            let contents = fs::read_to_string("res\\rustman5.txt")
                .expect("File rustman5.txt could not be opened!");
            println!("{}", contents);
            println!("5 guesses left");
        }
        4 => {
            let contents = fs::read_to_string("res\\rustman4.txt")
                .expect("File rustman4.txt could not be opened!");
            println!("{}", contents);
            println!("4 guesses left");
        }
        3 => {
            let contents = fs::read_to_string("res\\rustman3.txt")
                .expect("File rustman3.txt could not be opened!");
            println!("{}", contents);
            println!("3 guesses left");
        }
        2 => {
            let contents = fs::read_to_string("res\\rustman2.txt")
                .expect("File rustman2.txt could not be opened!");
            println!("{}", contents);
            println!("2 guesses left");
        }
        1 => {
            let contents = fs::read_to_string("res\\rustman1.txt")
                .expect("File rustman1.txt could not be opened!");
            println!("{}", contents);
            println!("1 guesses left");
        }
        _ => (),
    }

    Ok(())
} // fn print_rustman

/// ask the player for a letter
fn ask(guessed_letters: &mut String) -> io::Result<char> {
    println!("Please guess a letter");
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            let v: Vec<_> = guessed_letters.match_indices(c).collect();
            if !v.is_empty() {
                println!("You have already guessed {}. Try again.", guessed_letters);
                continue;
            }
            if c.is_alphabetic() && c.is_lowercase() {
                guessed_letters.push(c);
                return Ok(c);
            }
        }
    }
}

fn check_guess(letter: char, word: &str) -> Vec<(usize, &str)> {
    let v: Vec<_> = word.match_indices(letter).collect();
    v
}

fn fill_answer(hits: Vec<(usize, &str)>, answer: &mut String) -> String {
    let mut new_answer = String::new();
    for i in 0..answer.len() {
        let mut pushed = false;
        for hit in &hits {
            if hit.0 == i {
                new_answer.push(hit.1.chars().next().unwrap());
                pushed = true;
            }
        }
        if !pushed {
            new_answer.push(answer.chars().nth(i).unwrap());
        }
    }
    new_answer
}

fn print_answer(answer: &String) -> io::Result<()> {
    let mut answer_display = String::new();
    for i in 0..answer.len() {
        answer_display.push(answer.chars().nth(i).unwrap());
        answer_display.push(' ');
    }
    println!("{}", answer_display);
    Ok(())
}

fn print_lose(word: String) {
    println!("Rustman rusted away. The word was {}. You lose :(", word);
}

fn print_win(word: String) {
    println!("You saved Rustman!! Congratulations! The word was {}", word);
}
