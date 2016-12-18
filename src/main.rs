use std::fs::File;
use std::io::Read;
use ParseToken as PT;

fn main() {
    match slurp("test2.lisp") {
        Some(txt) => {
            let mut ts = Vec::new();
            ts = txt.chars().map(|c| parse(c)).collect();
            println!("{:?}", ts);
        }
        _ => {}
    }
}

/// Slurp a file like clojure.
fn slurp(file: &str) -> Option<String> {
    let mut s = String::new();
    File::open(file)
        .ok()
        .and_then(|mut x| x.read_to_string(&mut s).ok())
        .and(Some(s))
}

/// Convert the characters in a file into ParseTokens.
///
/// The tokens add the first level of structure to the file and will be passed
/// along to a statemachine to build up the AST.
fn parse (c: char) -> ParseToken {
    match c {
       '(' => PT::BeginExpr,
       ')' => PT::EndExpr,
       ' ' => PT::Space,
       '\n'=> PT::Newline,
       '\t'=> PT::Tab,
       _   => PT::Unfinished(c)
    }
}

#[derive(Debug)]
enum ParseToken {
    BeginExpr,
    EndExpr,
    Space,
    Newline,
    Tab,
    Unfinished(char),
}

#[test]
fn test_slurp() {
    assert_eq!(slurp("test.lisp"), Some(String::from("(+ 1 2)\n")));
}
