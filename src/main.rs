use std::fs::File;
use std::io::Read;
use ParseToken as PT;

fn main() {
    match slurp("test2.lisp") {
        Some(txt) => {
            let ts: Vec<ParseToken> = txt.chars().map(|c| parse(c)).collect();
            println!("{:?}", ts);
        }
        _ => {println!("{}", "No file found.")}
    }
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

fn stringify (xs: Vec<ParseToken>) -> String {
    xs.iter().map(|x| match x { &PT::Unfinished(c) => Some(c), _ => None})
             .map(|x| x.unwrap())
             .collect()
}

/// Slurp a file like clojure.
fn slurp(file: &str) -> Option<String> {
    let mut s = String::new();
    File::open(file)
        .ok()
        .and_then(|mut x| x.read_to_string(&mut s).ok())
        .and(Some(s))
}

#[test]
fn test_slurp() {
    assert_eq!(slurp("test.lisp"), Some(String::from("(+ 1 2)\n")));
}

#[test]
fn test_stringify() {
    assert_eq!(stringify(vec![PT::Unfinished('f'),
                              PT::Unfinished('o'),
                              PT::Unfinished('o')]),
               String::from("foo"));
}
