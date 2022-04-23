#![allow(unused_imports)]
use super::lexer_test;
use crate::lexer::{
    tokenize,
    tokens::{TargetType, TokenType},
};
use std::str::FromStr;

// Basic tokens tests
lexer_test!(tokenize_single_integer, "32" => vec![TokenType::Push(TargetType::Integer(32))]);
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
lexer_test!(tokenize_while, "while" => vec![TokenType::While]);
lexer_test!(tokenize_do, "do" => vec![TokenType::Do]);
lexer_test!(tokenize_mem, "mem" => vec![TokenType::Mem]);

// Push/Pop tests
lexer_test!(tokenize_pop_to_memory, "@" => vec![TokenType::Pop(TargetType::Memory)]);
lexer_test!(tokenize_push_from_memory, "!" => vec![TokenType::Push(TargetType::Memory)]);
lexer_test!(tokenize_push_integer, "!1" => vec![TokenType::Push(TargetType::Integer(1))]);
lexer_test!(tokenize_pop_to_register, "@r1" => vec![TokenType::Pop(TargetType::Regsiter(1))]);
lexer_test!(tokenize_push_from_register, "!r1" => vec![TokenType::Push(TargetType::Regsiter(1))]);

// Identifiers tests
lexer_test!(tokenize_single_word_identifier, "test" => vec![TokenType::Identifier("test".to_string())]);
lexer_test!(tokenize_long_identifier, "test_test" => vec![TokenType::Identifier("test_test".to_string())]);
lexer_test!(tokenize_underscore_identifier, "_test" => vec![TokenType::Identifier("_test".to_string())]);

// Program tests
lexer_test!(tokenize_program, "12 123 + - . test" => vec![
    TokenType::Push(TargetType::Integer(12)),
    TokenType::Push(TargetType::Integer(123)),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_multiple_spaces_program, "   12     123 + - .      test   " => vec![
    TokenType::Push(TargetType::Integer(12)),
    TokenType::Push(TargetType::Integer(123)),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_tabbed_program, "\t12\t123\t+\t-\t.\ttest\t" => vec![
    TokenType::Push(TargetType::Integer(12)),
    TokenType::Push(TargetType::Integer(123)),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_multiple_lines_program, "12 123 + - . test\n12 123 + - . test" => vec![
    TokenType::Push(TargetType::Integer(12)),
    TokenType::Push(TargetType::Integer(123)),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
    TokenType::Push(TargetType::Integer(12)),
    TokenType::Push(TargetType::Integer(123)),
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Dot,
    TokenType::Identifier("test".to_string()),
]);
lexer_test!(tokenize_if_else_program, "1 if 2 else 3 end" => vec![
    TokenType::Push(TargetType::Integer(1)),
    TokenType::If,
    TokenType::Push(TargetType::Integer(2)),
    TokenType::Else,
    TokenType::Push(TargetType::Integer(3)),
    TokenType::End,
]);
lexer_test!(tokenize_while_program, "while 1 do 2 . end" => vec![
    TokenType::While,
    TokenType::Push(TargetType::Integer(1)),
    TokenType::Do,
    TokenType::Push(TargetType::Integer(2)),
    TokenType::Dot,
    TokenType::End,
]);
lexer_test!(tokenize_memory_program, "mem 1 @ mem ! ." => vec![
    TokenType::Mem,
    TokenType::Push(TargetType::Integer(1)),
    TokenType::Pop(TargetType::Memory),
    TokenType::Mem,
    TokenType::Push(TargetType::Memory),
    TokenType::Dot,
]);

// Comments tests
lexer_test!(tokenize_single_line_comment1, "12 // comment" => vec![TokenType::Push(TargetType::Integer(12))]);
lexer_test!(tokenize_single_line_comment2, "12 /* comment */" => vec![TokenType::Push(TargetType::Integer(12))]);
lexer_test!(tokenize_multiple_lines_comment1, "12 // comment\n12 // comment" => vec![
    TokenType::Push(TargetType::Integer(12)), TokenType::Push(TargetType::Integer(12))
]);
lexer_test!(tokenize_multiple_lines_comment2, "/* comment\ncomment */ 12" => vec![TokenType::Push(TargetType::Integer(12))]);

// Fail tests
lexer_test!(FAIL: tokenize_invalid_operator, "+-");
lexer_test!(FAIL: tokenize_invalid_identifier, ".test");
lexer_test!(FAIL: tokenize_invalid_number, "32asd");
lexer_test!(FAIL: tokenize_inavlid_pop, "@1");
lexer_test!(FAIL: tokenize_inavlid_pop_register_1, "@ra1");
lexer_test!(FAIL: tokenize_inavlid_pop_register_2, "@r1a");
lexer_test!(FAIL: tokenize_inavlid_push_register_1, "!ra1");
lexer_test!(FAIL: tokenize_inavlid_push_register_2, "!r1a");
