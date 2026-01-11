use xjtu_codegen::lexer::Lexer;
use xjtu_codegen::token::Token;
use std::fs::File;
use std::io::Read;

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

    let expected = "(1)\tKeywords(Int)
(2)\tIdent(\"main\")
(3)\tLParam
(4)\tRParam
(5)\tLBrace
(6)\tKeywords(Int)
(7)\tIdent(\"x\")
(8)\tAssign
(9)\tInt(\"1\")
(10)\tSemicolon
(11)\tKeywords(If)
(12)\tLParam
(13)\tIdent(\"x\")
(14)\tGt
(15)\tInt(\"0\")
(16)\tRParam
(17)\tLBrace
(18)\tKeywords(Int)
(19)\tIdent(\"y\")
(20)\tAssign
(21)\tInt(\"2\")
(22)\tSemicolon
(23)\tRBrace
(24)\tKeywords(Int)
(25)\tIdent(\"y\")
(26)\tAssign
(27)\tInt(\"0\")
(28)\tSemicolon
(29)\tIdent(\"x\")
(30)\tAssign
(31)\tIdent(\"x\")
(32)\tPlus
(33)\tIdent(\"y\")
(34)\tTimes
(35)\tInt(\"2\")
(36)\tMinus
(37)\tInt(\"5\")
(38)\tSemicolon
(39)\tKeywords(Int)
(40)\tIdent(\"a\")
(41)\tAssign
(42)\tInt(\"10\")
(43)\tSemicolon
(44)\tKeywords(While)
(45)\tLParam
(46)\tIdent(\"a\")
(47)\tGt
(48)\tInt(\"0\")
(49)\tRParam
(50)\tLBrace
(51)\tIdent(\"a\")
(52)\tAssign
(53)\tIdent(\"a\")
(54)\tMinus
(55)\tInt(\"1\")
(56)\tSemicolon
(57)\tRBrace
(58)\tKeywords(Return)
(59)\tInt(\"0\")
(60)\tSemicolon
(61)\tRBrace
";

    assert_eq!(out, expected);
}
