use logos::Logos;

// 1. THE ZERO-ALLOCATION LEXER & INTERNER
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
// We explicitly tell Logos this is intentional!
#[logos(skip(r"--[^\n]*", allow_greedy = true))]
pub enum Token<'a> {
    #[token("local")]
    Local,
    #[token("=")]
    Assign,
    #[token("do")]
    Do,
    #[token("end")]
    End,
    #[token(".")]
    Dot,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Nil,
    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("elseif")]
    ElseIf,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("<")]
    LessThan,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),

    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),

    #[regex(r#""[^"]*""#)]
    String(&'a str),
}
