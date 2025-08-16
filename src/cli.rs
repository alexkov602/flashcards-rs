use rand::seq::SliceRandom;
use std::io::{self, Write};

use crate::{flashcard::Flashcard, storage};

pub fn run() -> Result<(), io::Error> {
    let path = "cards.txt";

    loop {
        println!(
            r#"
Choose an action:
1) Add a card
2) List cards
3) Quiz yourself
4) Exit
"#
        );

        print!("Enter choice: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => add(path)?,
            "2" => list(path),
            "3" => quiz(path),
            "4" => exit(),
            _ => println!("Invalid choice, try again!"),
        }
    }
}

fn add(path: &str) -> Result<(), io::Error> {
    let mut question = String::new();
    let mut answer = String::new();

    print!("\nEnter the question: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut question)?;

    print!("Enter the answer: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut answer)?;

    let card = Flashcard::new(&question, &answer);
    storage::save_card(path, &card)
}

fn list(path: &str) {
    let cards = storage::load_cards(path);

    if cards.is_empty() {
        println!("No cards available.");
    } else {
        for (i, card) in cards.iter().enumerate() {
            println!("{}. Q: {}\n   A: {}\n", i + 1, card.question, card.answer);
        }
    }
}

fn quiz(path: &str) {
    let mut cards = storage::load_cards(path);
    if cards.is_empty() {
        println!("No cards to quiz.");
        return;
    }

    let mut rng = rand::rng();
    cards.shuffle(&mut rng);

    for card in cards {
        println!("Question: {}", card.question);
        println!("Press Enter to reveal the answer...");
        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy).unwrap();
        println!("Answer: {}\n", card.answer);
    }
}

fn exit() {
    println!("Goodbye!");
    std::process::exit(0);
}
