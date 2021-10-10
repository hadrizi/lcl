use crate::scanner::defs::TokenName;

mod scanner;

fn scanfile(filename: String) {
    let mut scanner = scanner::scan::Scanner::new(filename);
    let mut token = scanner::defs::Token::new();
    while scanner.scan(&mut token) {
        print!("Token {}", token.token_name.value());
        if token.token_name == TokenName::INTLIT {
            print!(", value {}", token.int_value);
        }
        println!();
    }
}

fn main(){
    scanfile(String::from("sources/input05"));
    println!();
    // scanfile(String::from("sources/input02"));
    // println!();
    // scanfile(String::from("sources/input03"));
    // println!();
    // scanfile(String::from("sources/input04"));
    // println!();
    // scanfile(String::from("sources/input05"));
}