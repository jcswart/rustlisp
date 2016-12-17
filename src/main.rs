use std::fs::File;
use std::io::Read;

fn main() {
    match slurp("test.lisp") {
        Some(txt) => {
            let mut ts = Vec::new();
            for c in txt.chars() {
                let t = parse(c);
                match t {
                    Token::Newline | Token::Space => { continue; },
                    _ => { ts.push(t); },
                }
            }
            println!("{:?}", ts);
        }
        _ => { println!("Couldn't read file."); }
    }
}

fn slurp(file: &str) -> Option<String> {
    let mut s = String::new();
    let f     = File::open(file).ok();
    if f.is_some() {
        f.unwrap().read_to_string(&mut s);
        Some(s)
    } else {
        None
    }
}

fn parse (c: char) -> Token {
    match c {
       '(' => Token::BeginExpr,
       ')' => Token::EndExpr,
       ' ' => Token::Space,
       '\n'=> Token::Newline,
       _   => Token::Lookup(c)
    }
}

#[derive(Debug)]
enum Token {
    BeginExpr,
    EndExpr,
    Space,
    Newline,
    Lookup(char),
}

#[test]
fn test_slurp() {
    assert_eq!(slurp("test.lisp"), Some(String::from("(+ 1 2)\n")));
}
