use crate::functions::eval;
use std::{collections::HashMap, fmt, rc::Rc};

pub type Context = HashMap<String, Slisp>;

#[derive(Clone)]
pub struct LispFunction(pub Rc<Box<dyn Fn(&mut Context, Vec<Slisp>) -> Slisp>>);

#[derive(Clone)]
pub enum Slisp {
    Func(LispFunction),
    Numeric(i32),
    Atom(String),
    List(Vec<Slisp>),
    None,
}

pub fn get_value(context: &mut Context, literal: Slisp) -> Slisp {
    match literal {
        Slisp::Atom(s) => {
            if let Ok(x) = s.parse::<i32>() {
                Slisp::Numeric(x).into()
            } else if let Some(value) = context.get(&s) {
                value.clone()
            } else {
                Slisp::None
            }
        }
        Slisp::List(l) => eval(context, l),
        x => x,
    }
}

impl fmt::Debug for Slisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Slisp::Func(_) => f.write_str("Slisp::Func(#Func#)"),
            Slisp::Numeric(n) => f.write_str(&format!("Slisp::Numeric({})", n)),
            Slisp::Atom(s) => f.write_str(&format!("Slisp::Atom({})", s)),
            Slisp::List(v) => f.write_str(&format!("Slisp::List({:?})", v)),
            Slisp::None => f.write_str("Slisp::None"),
        }
    }
}

impl fmt::Display for Slisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Slisp::Func(_) => f.write_str(&format!("#Func#")),
            Slisp::Numeric(n) => f.write_str(&format!("{}", n)),
            Slisp::Atom(s) => f.write_str(&format!("{}", s)),
            Slisp::List(l) => f.write_str(&format!(
                "({})",
                l.iter()
                    .map(|elem| format!("{}", elem))
                    .collect::<Vec<String>>()
                    .join(" ")
            )),
            Slisp::None => f.write_str(&format!("nil")),
        }
    }
}
