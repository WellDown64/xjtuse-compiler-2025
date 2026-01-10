use xjtu_parser::lexer::Lexer;
use xjtu_parser::token::Token;
use std::fs::File;
use std::io::Read;
use std::fmt;

#[test]
fn test_example() {
    let mut file = File::open("tests/example.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let lexer = Lexer::new(&input);
    let tokens = lexer.to_tokens().unwrap();

    let mut out = String::new();
    for (i, t) in tokens.into_iter().filter(|t| *t != Token::EOF).enumerate() {
        out.push_str(&format!("({})\t{:?}\n", i + 1, t));
    }

    let expected = "(1)	Keywords(Int)
(2)	Ident(\"main\")
(3)	LParam
(4)	RParam
(5)	LBrace
(6)	Keywords(Int)
(7)	Ident(\"x\")
(8)	Assign
(9)	Int(\"1\")
(10)	Semicolon
(11)	Keywords(If)
(12)	LParam
(13)	Ident(\"x\")
(14)	Gt
(15)	Int(\"0\")
(16)	RParam
(17)	LBrace
(18)	Keywords(Int)
(19)	Ident(\"y\")
(20)	Assign
(21)	Int(\"2\")
(22)	Semicolon
(23)	RBrace
(24)	Ident(\"x\")
(25)	Assign
(26)	Ident(\"x\")
(27)	Plus
(28)	Ident(\"y\")
(29)	Times
(30)	Int(\"2\")
(31)	Minus
(32)	Int(\"5\")
(33)	Semicolon
(34)	Keywords(Int)
(35)	Ident(\"a\")
(36)	Assign
(37)	Int(\"10\")
(38)	Semicolon
(39)	Keywords(While)
(40)	LParam
(41)	Ident(\"a\")
(42)	Gt
(43)	Int(\"0\")
(44)	RParam
(45)	LBrace
(46)	Ident(\"a\")
(47)	Assign
(48)	Ident(\"a\")
(49)	Minus
(50)	Int(\"1\")
(51)	Semicolon
(52)	RBrace
(53)	Keywords(Return)
(54)	Int(\"0\")
(55)	Semicolon
(56)	RBrace
";

    assert_eq!(out, expected);
}
