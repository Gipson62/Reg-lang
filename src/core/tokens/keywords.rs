use crate::core::tokens::tokens_type::TokenType;

#[derive(Clone, Debug, PartialEq)]
/// Struct representing a `Keyword`.
pub(crate) struct Keyword {
    pub name: String,
    pub tok_type: TokenType,
}
impl Keyword {
    /// Creates a new `Keyword` instance.
    pub fn new(name: String, tok_type: TokenType) -> Keyword {
        Keyword {
            name,
            tok_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq,)]
/// List of all the existing `Keywords`.
pub(crate) struct Keywords {
    pub keywords: Vec<Keyword>,
}
impl Keywords {
    /// Creates a new `Keywords` instance.
    pub fn new() -> Keywords {
        Keywords {
            keywords: vec![
                Keyword::new(
                    String::from("continue"),
                    TokenType::TTContinue,
                ),
                Keyword::new(
                    String::from("break"),
                    TokenType::TTBreak,
                ),
                Keyword::new(
                    String::from("struct"),
                    TokenType::TTStruct,
                ),
                Keyword::new(
                    String::from("loop"),
                    TokenType::TTLoop,
                ),
                Keyword::new(
                    String::from("while"),
                    TokenType::TTWhile,
                ),
                Keyword::new(
                    String::from("var"),
                    TokenType::TTVar,
                ),
                Keyword::new(
                    String::from("true"),
                    TokenType::TTTrue,
                ),
                Keyword::new(
                    String::from("this"),
                    TokenType::TTThis,
                ),
                Keyword::new(
                    String::from("super"),
                    TokenType::TTSuper,
                ),
                Keyword::new(
                    String::from("return"),
                    TokenType::TTReturn,
                ),
                Keyword::new(
                    String::from("print"),
                    TokenType::TTPrint,
                ),
                Keyword::new(
                    String::from("or"),
                    TokenType::TTOr,
                ),
                Keyword::new(
                    String::from("none"),
                    TokenType::TTNone,
                ),
                Keyword::new(
                    String::from("if"),
                    TokenType::TTIf,
                ),
                Keyword::new(
                    String::from("for"),
                    TokenType::TTFor,
                ),
                Keyword::new(
                    String::from("func"),
                    TokenType::TTFunc,
                ),
                Keyword::new(
                    String::from("false"),
                    TokenType::TTFalse,
                ),
                Keyword::new(
                    String::from("else"),
                    TokenType::TTElse,
                ),
                Keyword::new(
                    String::from("class"),
                    TokenType::TTClass,
                ),
                Keyword::new(
                    String::from("and"),
                    TokenType::TTAnd,
                ),
            ],
        }
    }
}