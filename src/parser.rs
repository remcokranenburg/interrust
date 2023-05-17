use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Value {
    I32(i32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::I32(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub struct Assignment {
    name: String,
    value: Expr,
}

peg::parser! {
    pub grammar rust_parser() for str {
        pub rule assignment() -> Assignment
            = "let" __ "mut"? __ n:identifier() __ "=" __ v:expr() __ ";" __ {
                Assignment {
                    name: n,
                    value: v,
                }
            }

        pub rule expr() -> Expr = precedence!{
            x:(@) __ "+" __ y:@ { Expr::Add(Box::new(x), Box::new(y)) }
            x:(@) __ "-" __ y:@ { Expr::Sub(Box::new(x), Box::new(y)) }
            --
            x:(@) __ "*" __ y:@ { Expr::Mul(Box::new(x), Box::new(y)) }
            x:(@) __ "/" __ y:@ { Expr::Div(Box::new(x), Box::new(y)) }
            --
            "-" x:@ { Expr::Neg(Box::new(x)) }
            v:value() { Expr::Value(v) }
            "(" __ e:expr() __ ")" { e }
        }

        pub rule identifier() -> String
            = i:$(['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {
                i.to_owned()
            }

        pub rule value() -> Value
            = v:number() { v }

        rule number() -> Value
            = n:$(['-' | '+']? ['0'..='9']+) { Value::I32(n.parse().unwrap()) }

        rule __
            = [' ' | '\t' | '\n']*
    }
}

#[test]
fn test_number() {
    let actual = rust_parser::value("01234");
    let expected = Ok(Value::I32(1234));
    assert_eq!(actual, expected);
}

#[test]
fn test_add() {
    let actual = rust_parser::expr("1+2");
    let expected = Ok(Expr::Add(Box::new(Expr::Value(Value::I32(1))), Box::new(Expr::Value(Value::I32(2)))));
    assert_eq!(actual, expected);
}
