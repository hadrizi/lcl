#![allow(unused_imports)]
use super::lexer_test;
use crate::lexer::{tokenize, tokens::TokenType};
use std::str::FromStr;

// Basic tokens tests
lexer_test!(tokenize_single_integer, "32" => vec![TokenType::Integer(32)]);
lexer_test!(tokenize_plus, "+" => vec![TokenType::Plus]);
lexer_test!(tokenize_minus, "-" => vec![TokenType::Minus]);
lexer_test!(tokenize_dot, "." => vec![TokenType::Dot]);
lexer_test!(tokenize_greater, ">" => vec![TokenType::Greater]);
lexer_test!(tokenize_less, "<" => vec![TokenType::Less]);
lexer_test!(tokenize_equal, "=" => vec![TokenType::Equal]);
lexer_test!(tokenize_notequal, "!=" => vec![TokenType::NotEqual]);
lexer_test!(tokenize_if, "if" => vec![TokenType::If]);
lexer_test!(tokenize_else, "else" => vec![TokenType::Else]);
lexer_test!(tokenize_end, "end" => vec![TokenType::End]);

// Identifiers tests
lexer_test!(tokenize_single_word_identifier, "test" => vec![TokenType::Identifier("test".to_string())]);
lexer_test!(tokenize_long_identifier, "test_test" => vec![TokenType::Identifier("test_test".to_string())]);
lexer_test!(tokenize_underscore_identifier, "_test" => vec![TokenType::Identifier("_test".to_string())]);

// Program tests
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
lexer_test!(tokenize_if_else_program, "1 if 2 else 3 end" => vec![
    TokenType::Integer(1),
    TokenType::If,
    TokenType::Integer(2),
    TokenType::Else,
    TokenType::Integer(3),
    TokenType::End,
]);

// Comments tests
lexer_test!(tokenize_single_line_comment1, "12 // comment" => vec![TokenType::Integer(12)]);
lexer_test!(tokenize_single_line_comment2, "12 /* comment */" => vec![TokenType::Integer(12)]);
lexer_test!(tokenize_multiple_lines_comment1, "12 // comment\n12 // comment" => vec![
    TokenType::Integer(12), TokenType::Integer(12)
]);
lexer_test!(tokenize_multiple_lines_comment2, "/* comment\ncomment */ 12" => vec![TokenType::Integer(12)]);

// Fail tests
lexer_test!(FAIL: tokenize_invalid_operator, "+-");
lexer_test!(FAIL: tokenize_invalid_identifier, ".test");
lexer_test!(FAIL: tokenize_invalid_number, "32asd");
