#![allow(unused_imports)]
use super::lexer_test;
use crate::lexer::{tokenize, tokens::TokenType};
use std::str::FromStr;

lexer_test!(tokenize_single_integer, "32" => vec![TokenType::Integer(32)]);
lexer_test!(tokenize_plus, "+" => vec![TokenType::Plus]);
lexer_test!(tokenize_minus, "-" => vec![TokenType::Minus]);
lexer_test!(tokenize_dot, "." => vec![TokenType::Dot]);
lexer_test!(tokenize_single_word_identifier, "test" => vec![TokenType::Identifier("test".to_string())]);
lexer_test!(tokenize_long_identifier, "test_test" => vec![TokenType::Identifier("test_test".to_string())]);
lexer_test!(tokenize_underscore_identifier, "_test" => vec![TokenType::Identifier("_test".to_string())]);
lexer_test!(tokenize_program, "12 123 + - . test" => vec![
    TokenType::Integer(12),
    TokenType::Integer(123),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_multiple_spaces_program, "   12     123 + - .      test   " => vec![
    TokenType::Integer(12),
    TokenType::Integer(123),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_tabbed_program, "\t12\t123\t+\t-\t.\ttest\t" => vec![
    TokenType::Integer(12),
    TokenType::Integer(123),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_multiple_lines_program, "12 123 + - . test\n12 123 + - . test" => vec![
    TokenType::Integer(12),
    TokenType::Integer(123),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
    TokenType::Integer(12),
    TokenType::Integer(123),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_single_line_comment1, "12 // comment" => vec![TokenType::Integer(12)]);
lexer_test!(tokenize_single_line_comment2, "12 /* comment */" => vec![TokenType::Integer(12)]);
lexer_test!(tokenize_multiple_lines_comment1, "12 // comment\n12 // comment" => vec![
    TokenType::Integer(12), TokenType::Integer(12)
]);
lexer_test!(tokenize_multiple_lines_comment2, "/* comment\ncomment */ 12" => vec![TokenType::Integer(12)]);

lexer_test!(FAIL: tokenize_invalid_operator, "+-");
lexer_test!(FAIL: tokenize_invalid_identifier, ".test");
lexer_test!(FAIL: tokenize_invalid_number, "32asd");
