use std::{collections::HashMap, fmt};

pub type Context = HashMap<String, Slisp>;
pub struct LispFunction(pub Box<dyn Fn(&mut Context, Vec<LispFunction>) -> Slisp>);

pub enum Slisp {
    Func(LispFunction),
    Numeric(i32),
    String(String),
    None,
}

impl From<Slisp> for LispFunction {
    fn from(value: Slisp) -> Self {
        match value {
            Slisp::Func(function) => function,
            Slisp::Numeric(x) => LispFunction(Box::new(
                move |_context: &mut Context, _args: Vec<LispFunction>| Slisp::Numeric(x),
            )),
            Slisp::String(s) => LispFunction(Box::new(
                move |_context: &mut Context, _args: Vec<LispFunction>| Slisp::String(s.clone()),
            )),
            Slisp::None => LispFunction(Box::new(
                |_context: &mut Context, _args: Vec<LispFunction>| Slisp::None,
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LispStruct {
    String(String),
    Struct(Box<Vec<LispStruct>>),
}

impl fmt::Debug for Slisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Slisp::Func(_) => f.write_str("Slisp::Func(#Func#)"),
            Slisp::Numeric(n) => f.write_str(&format!("Slisp::Numeric({})", n)),
            Slisp::String(s) => f.write_str(&format!("Slisp::String({})", s)),
            Slisp::None => f.write_str("Slisp::None"),
        }
    }
}

impl fmt::Display for Slisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Slisp::Func(_) => panic!("Can't format properly a function"),
            Slisp::Numeric(n) => f.write_str(&format!("{}", n)),
            Slisp::String(s) => f.write_str(&format!("{}", s)),
            Slisp::None => panic!("Can't format nil"),
        }
    }
}
