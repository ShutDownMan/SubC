#![warn(rustdoc::missing_crate_level_docs)]
#![warn(missing_docs)]
//! # Matching Brackets
//!
//! Code taken from following page: `<https://codereview.stackexchange.com/questions/253279/matching-brackets-in-rust>`

enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn from_char(c: char) -> Option<Bracket> {
        match c {
            '{' | '[' | '(' => Some(Bracket::Open(c)),
            '}' => Some(Bracket::Close('{')),
            ']' => Some(Bracket::Close('[')),
            ')' => Some(Bracket::Close('(')),
            _ => None,
        }
    }
}

pub enum IsBalanced {
    Balanced,
    Unbalanced(usize),
}

/// Checka se a entrada `string` tem brackets balanceados.
///
/// # Exemplos
///
/// ```rust
/// use matching_brackets::brackets_are_balanced;
/// assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"), matching_brackets::IsBalanced::Balanced);
/// assert!(brackets_are_balanced("([{}({}[])])"), matching_brackets::IsBalanced::Balanced);
/// assert!(!brackets_are_balanced("{[)][]}", matching_brackets::IsBalanced::Balanced))
/// ```
pub fn brackets_are_balanced(string: &str) -> IsBalanced {
    let mut brackets: Vec<char> = vec![];
    for (index, c) in string.chars().enumerate() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(char_bracket);
            }
            Some(Bracket::Close(char_close_bracket)) => {
                if brackets.pop() != Some(char_close_bracket) {
                    return IsBalanced::Unbalanced(index);
                }
            }
            _ => {}
        }
    }
    if brackets.is_empty() {
        IsBalanced::Balanced
    } else {
        IsBalanced::Unbalanced(string.len())
    }
}
