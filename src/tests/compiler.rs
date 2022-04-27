#![allow(unused_imports)]
use super::compiler_test;
use crate::compiler::compile;
use crate::lexer::{tokenize, tokens::Token};
use std::fs::remove_file;
use std::process::Command;
use std::str::from_utf8;

// Push/Pop
compiler_test!(compile_push_int, "!1 ." => "1\n");
compiler_test!(compile_registers, "1 @r1 !r1 ." => "1\n");

// Arithmetics
compiler_test!(compile_plus, "2 2 + ." => "4\n");
compiler_test!(compile_minus, "5 2 - ." => "3\n");

// Comparison
compiler_test!(compile_less_true, "1 2 < ." => "1\n");
compiler_test!(compile_greater_true, "2 1 > ." => "1\n");
compiler_test!(compile_equal_true, "2 2 = ." => "1\n");
compiler_test!(compile_not_equal_true, "2 3 != ." => "1\n");
compiler_test!(compile_less_false, "2 1 < ." => "0\n");
compiler_test!(compile_greater_false, "1 2 > ." => "0\n");
compiler_test!(compile_equal_false, "2 3 = ." => "0\n");
compiler_test!(compile_not_equal_false, "2 2 != ." => "0\n");

// Stack manipulation
compiler_test!(compile_dup, "1 dup . ." => "1\n1\n");
compiler_test!(compile_drop, "1 2 drop ." => "1\n");
compiler_test!(compile_swap, "1 2 swap . ." => "1\n2\n");
compiler_test!(compile_over, "1 2 over . . ." => "1\n2\n1\n");
compiler_test!(compile_rot, "1 2 3 rot . . ." => "1\n3\n2\n");

// Memory
compiler_test!(compile_memory_1, "mem 1 @ mem ! ." => "1\n");
compiler_test!(compile_memory_2, "mem 8 + 2 @ mem 8 + ! ." => "2\n");

// Control flow
compiler_test!(
    compile_if,
    "2 2 = if
        1 . 
    end" => "1\n"
);
compiler_test!(
    compile_else,
    "2 2 != if
        1 . 
    else 
        2 . 
    end" => "2\n"
);
compiler_test!(
    compile_while,
    "0
    while dup 10 < do 
        dup . 
        1 + 
    end" => "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n"
);

// Functions
compiler_test!(
    compile_function_without_args,
    "fn test do
        1 .
    end
    test" => "1\n"
);
compiler_test!(
    compile_function_with_args,
    "fn add a b do
        a b +
    end
    3 2 add ." => "5\n"
);

compiler_test!(FAIL: unexpected_else, "1 2 3 else 1 2 3");
compiler_test!(FAIL: unexpected_do, "1 2 3 do 1 2 3");
compiler_test!(FAIL: unexpected_end_of_block, "1 2 3 end 1 2 3");
compiler_test!(FAIL: not_defined, "asd");
compiler_test!(FAIL: unfinished_function, "fn test do");
