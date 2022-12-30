use std::str::Chars;

pub fn lexer(input: &mut String) -> Chars<'_> {
    input.trim().chars()
}
