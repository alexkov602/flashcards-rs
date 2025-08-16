use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::flashcard::Flashcard;

fn parse_line(line: &str) -> Option<Flashcard> {
    let line = line.trim();
    if line.is_empty() || !line.contains('|') {
        return None;
    }

    let parts: Vec<&str> = line.splitn(2, '|').collect();
    if let [question, answer] = &parts[..] {
        Some(Flashcard::new(question, answer))
    } else {
        None
    }
}

pub fn load_cards(path: &str) -> Vec<Flashcard> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };

    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok) // skip lines that failed to read
        .filter_map(|line| parse_line(&line)) // parse lines into Flashcards
        .collect()
}

pub fn save_card(path: &str, flashcard: &Flashcard) -> Result<(), std::io::Error> {
    let file = OpenOptions::new().append(true).create(true).open(path)?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}|{}", flashcard.question, flashcard.answer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn parse_line_works() {
        let question = "What is Rust?";
        let answer = "Rust is a systems programming language!";

        let result = parse_line(&format!("{}|{}", question, answer));

        match result {
            Some(card) => {
                assert_eq!(card, Flashcard::new(question, answer));
            }
            None => {}
        }
    }

    #[test]
    fn load_cards_works() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();

        let cards = vec![
            Flashcard::new("What is Rust?", "Rust is a systems programming language!"),
            Flashcard::new("Why is Rust superior to any language?", "Because it is!"),
        ];
        std::fs::write(
            path,
            format!(
                "{}|{}\n{}|{}",
                cards[0].question, cards[0].answer, cards[1].question, cards[1].answer
            ),
        )
        .unwrap();

        let result = load_cards(path);
        assert_eq!(result, cards);
    }

    #[test]
    fn save_card_works() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();

        let card = Flashcard::new("What is Rust?", "Rust is a systems programming language!");
        save_card(path, &card).unwrap();

        let contents = std::fs::read_to_string(path).unwrap();
        assert_eq!(
            contents.trim(),
            &format!("{}|{}", card.question, card.answer)
        );
    }
}
