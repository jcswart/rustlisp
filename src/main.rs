use std::fs::File;
use std::io::Read;
use std::collections::HashMap as HM;
use std::ops::Add;
use Token::*;
use LispVal::*;

fn main() {
    let source = slurp("test.lisp");
    if  source.is_none() {
        panic!("File not found!");
    }

    let results              = tokenize(source.unwrap());
    let values: Vec<LispVal> = results.into_iter().map(valueize).collect();
    let stacks               = generate_stacks(values);

    println!("{:?}", stacks);
}

fn generate_stacks(values: Vec<LispVal>) -> (Vec<LispVal>, Vec<Vec<LispVal>>) {
    let mut capture_func = false;
    let mut call = vec![];
    let mut args = vec![];
    let mut tmp = vec![];
    for v in values {
        match v {
            BeginExpr => { capture_func = true; },
            Atom(a)   => {
                if capture_func {
                    call.push(Atom(a));
                    capture_func = false;
                } else {
                    tmp.push(Atom(a));
                }
            },
            EndExpr => {
                args.push(tmp.clone());
                tmp.clear();
            }
            _ => {},
        }
    }
    (call, args)
}

fn tokenize (s: String) -> Vec<Token> {
    let text        = s;
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
    results
}

fn valueize_all (ts: Vec<Token>) -> Vec<LispVal> {
    ts.into_iter().map(valueize).collect()
}

fn valueize (t: Token) -> LispVal {
    match t {
        Char(c) => {
            match c {
                '('        => {BeginExpr},
                ')'        => {EndExpr},
                ' ' | '\n' => {Whitespace},
                _          => {Wildcard},
            }
        },
        Str(s)  => { Atom(s) },
    }
}

#[derive(Clone,Debug,PartialEq)]
enum LispVal {
    Atom(String),
    BeginExpr,
    EndExpr,
    Wildcard,
    Whitespace
}

/// Characters that denote whitespace or s-exprs.
fn is_terminal(c: char) -> bool {
    match c {
        '(' | ')' | ' ' | '\n' => { true },
        _                      => { false }
    }
}

#[derive(Clone,Debug,PartialEq)]
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

///
/// Tests
///

/// Test helper.
fn str(s: &str) -> String {
    String::from(s)
}

#[test]
fn test_slurp() {
    assert_eq!(slurp("test.lisp"), Some(str("(+ 1 2)\n")));
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

#[test]
fn test_tokenize() {
    assert_eq!(tokenize(str("(+ 1 2)")),
               vec![Char('('),
                    Str(str("+")),
                    Char(' '),
                    Str(str("1")),
                    Char(' '),
                    Str(str("2")),
                    Char(')')])
}

#[test]
fn test_valueize_all() {
    assert_eq!(valueize_all(tokenize(str("(+ 1 2)"))),
               vec![BeginExpr,
                    Atom(str("+")),
                    Whitespace,
                    Atom(str("1")),
                    Whitespace,
                    Atom(str("2")),
                    EndExpr])
}

#[test]
fn test_func_map_idea() {
    let mut functions = HM::<&str, fn(i64,i64) -> i64>::new();
    functions.insert("+", i64::add);
    let add = functions.get("+").unwrap();
    assert_eq!(3, add(1,2));
}

