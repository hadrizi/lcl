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

pub(crate) use lexer_test;
