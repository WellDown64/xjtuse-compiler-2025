use xjtu_lexer::lexer::Lexer;
use xjtu_lexer::token::Token;
use std::fs::File;
use std::io::Read;
use std::fmt;

#[test]
fn test_example() {
    let mut file = File::open("tests/example.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);
    let lexer = Lexer::new(&input);
    let tokens = lexer.to_tokens().unwrap();

    let mut out = String::new();
    for (i, t) in tokens.into_iter().filter(|t| *t != Token::EOF).enumerate() {
        out.push_str(&format!("({})\t({}, {})\n", i, t.get_id(), t.get_content()));
    }

    let expected = 
"(0)	(1, -)
(1)	(11, hello_world)
(2)	(4, -)
(3)	(11, k)
(4)	(21, -)
(5)	(11, m)
(6)	(21, -)
(7)	(11, n)
(8)	(22, -)
(9)	(5, -)
(10)	(20, -)
(11)	(2, -)
(12)	(11, k)
(13)	(23, -)
(14)	(12, 8)
(15)	(20, -)
(16)	(11, m)
(17)	(23, -)
(18)	(12, 5)
(19)	(20, -)
(20)	(11, n)
(21)	(23, -)
(22)	(11, k)
(23)	(13, -)
(24)	(11, m)
(25)	(20, -)
(26)	(6, -)
(27)	(11, n)
(28)	(18, -)
(29)	(12, 10)
(30)	(7, -)
(31)	(11, k)
(32)	(23, -)
(33)	(11, k)
(34)	(13, -)
(35)	(12, 1)
(36)	(20, -)
(37)	(3, -)
(38)	(20, -)
";

    assert_eq!(out, expected);
}
