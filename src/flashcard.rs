use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Flashcard {
    pub question: String,
    pub answer: String,
}

impl Flashcard {
    pub fn new(question: &str, answer: &str) -> Flashcard {
        Self {
            question: question.trim().to_string(),
            answer: answer.trim().to_string(),
        }
    }
}

impl fmt::Display for Flashcard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.question, self.answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_construction_works() {
        let question = "  What is Rust?  ";
        let answer = "  A systems programming language!  ";

        let result = Flashcard::new(question, answer);
        assert_eq!(
            result,
            Flashcard {
                question: question.trim().to_string(),
                answer: answer.trim().to_string(),
            }
        )
    }
}
