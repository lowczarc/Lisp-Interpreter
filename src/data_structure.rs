use std::{collections::HashMap, fmt, rc::Rc};

pub type Context = HashMap<String, Slisp>;

#[derive(Clone)]
pub struct LispFunction(pub Rc<Box<dyn Fn(&mut Context, Vec<LispFunction>) -> Slisp>>);

#[derive(Clone)]
pub enum Slisp {
    Func(LispFunction),
    Numeric(i32),
    Literal(String),
    List(Vec<Slisp>),
    None,
}

impl From<Slisp> for LispFunction {
    fn from(value: Slisp) -> Self {
        match value {
            Slisp::Func(function) => function,
            Slisp::Numeric(x) => LispFunction(Rc::new(Box::new(
                move |_context: &mut Context, _args: Vec<LispFunction>| Slisp::Numeric(x),
            ))),
            Slisp::Literal(s) => LispFunction(Rc::new(Box::new(
                move |_context: &mut Context, _args: Vec<LispFunction>| Slisp::Literal(s.clone()),
            ))),
            Slisp::List(v) => LispFunction(Rc::new(Box::new(
                move |_context: &mut Context, _args: Vec<LispFunction>| Slisp::List(v.clone()),
            ))),
            Slisp::None => LispFunction(Rc::new(Box::new(
                |_context: &mut Context, _args: Vec<LispFunction>| Slisp::None,
            ))),
        }
    }
}

pub fn get_value(context: &Context, literal: Slisp) -> Slisp {
    if let Slisp::Literal(s) = literal {
        if let Some(value) = context.get(&s) {
            value.clone()
        } else {
            Slisp::None
        }
    } else {
        literal
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
            Slisp::Literal(s) => f.write_str(&format!("Slisp::Literal({})", s)),
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
            Slisp::Literal(s) => f.write_str(&format!("{}", s)),
            Slisp::List(_) => f.write_str(&format!("#List#")),
            Slisp::None => f.write_str(&format!("#None#")),
        }
    }
}
