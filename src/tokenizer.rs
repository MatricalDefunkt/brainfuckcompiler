pub mod tokenizer {
    use std::str::Chars;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Tokens {
        Plus,
        Minus,
        Right,
        Left,
        LBracket,
        RBracket,
        Dot,
        Comma,
        Hash,
    }

    impl Tokens {
        pub fn to_string(&self) -> &'static str {
            match self {
                Tokens::Plus => "+",
                Tokens::Minus => "-",
                Tokens::Right => ">",
                Tokens::Left => "<",
                Tokens::LBracket => "[",
                Tokens::RBracket => "]",
                Tokens::Dot => ".",
                Tokens::Comma => ",",
                Tokens::Hash => "#",
            }
        }

        pub fn to_array() -> [Tokens; 9] {
            [
                Tokens::Plus,
                Tokens::Minus,
                Tokens::Right,
                Tokens::Left,
                Tokens::LBracket,
                Tokens::RBracket,
                Tokens::Dot,
                Tokens::Comma,
                Tokens::Hash,
            ]
        }
        pub fn tokenize(input: Chars<'_>) -> Vec<Tokens> {
            let mut tokens = vec![];
            for c in input {
                for token in Tokens::to_array().iter() {
                    if c.to_string() == token.to_string() {
                        tokens.push(*token);
                    }
                }
            }
            tokens
        }
    }
}
