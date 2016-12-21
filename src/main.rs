use std::fs::File;
use std::io::Read;
use Token::*;

fn main() {
    let file = slurp("test2.lisp");
    if  file.is_none() {
        panic!("Fuck");
    }

    let text        = file.unwrap();
    let mut chars   = text.chars().peekable();
    let mut results = Vec::<Token>::new();
    let mut buffer  = Vec::new();

    loop {
        let curr = chars.next();
        let next = chars.peek();
        if curr.is_none() {
            break;
        }
        let c = curr.unwrap();
        if is_terminal(c) {
            results.push(Char(c));
        } else {
            buffer.push(Char(c));
            if is_terminal(*next.unwrap_or(&' ')) {
                let tmp = buffer.clone();
                let xstr: String = tmp.iter().map(|x| match *x { Char(c) => {c}, _ => { ' ' }}).collect();
                results.push(Str(xstr));
                buffer.clear();
            }
        }
    }
    println!("{:?}", results);
}

/// Characters that denote whitespace or s-exprs.
fn is_terminal(c: char) -> bool {
    match c {
        '(' | ')' | ' ' | '\n' => { true },
        _ => { false }
    }
}

#[derive(Clone,Debug)]
enum Token {
    Char(char),
    Str(String)
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
fn test_is_terminal() {
    assert!(is_terminal('('));
    assert!(is_terminal(')'));
    assert!(is_terminal(' '));
    assert!(is_terminal('\n'));
    assert_eq!(false, is_terminal('a'));
    assert_eq!(false, is_terminal('.'));
}
