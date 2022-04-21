pub mod compiler;
pub mod lexer;

macro_rules! lexer_test {
    (FAIL: $name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;

            let got = tokenize(src, "<test>");
            assert!(got.is_err(), "{:?} should be an error", got);
        }
    };
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let should_be = $should_be;

            let got: Vec<TokenType> = tokenize(src, "<test>")
                .unwrap()
                .into_iter()
                .map(|x| x.ttype)
                .collect();
            assert_eq!(got, should_be, "Input was {:?}", src);
        }
    };
}

macro_rules! compiler_test {
    (FAIL: $name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let outfile = format!("src/tests/test_{}", stringify!($name));

            let mut tokens: Vec<Token> = tokenize(src, "<test>").unwrap();
            let result = compile(&mut tokens, &outfile);

            assert!(result.is_err(), "{:?} should be an error", result);
        }
    };
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let should_be = $should_be;
            let outfile = format!("src/tests/test_{}", stringify!($name));

            let mut tokens: Vec<Token> = tokenize(src, "<test>").unwrap();
            compile(&mut tokens, &outfile).unwrap();
            let output = Command::new(&outfile).output().unwrap();
            let result = from_utf8(&output.stdout).unwrap();

            remove_file(&outfile).unwrap();
            remove_file(format!("{}.o", &outfile)).unwrap();
            remove_file(format!("{}.asm", &outfile)).unwrap();

            assert_eq!(result, should_be, "Input was {:?}", src);
        }
    };
}

pub(crate) use compiler_test;
pub(crate) use lexer_test;
