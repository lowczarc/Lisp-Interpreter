use crate::functions::eval;
use std::{collections::HashMap, fmt, rc::Rc};

#[derive(Debug)]
pub struct Context {
    global: HashMap<String, Slisp>,
    scope: HashMap<String, Slisp>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            global: HashMap::new(),
            scope: HashMap::new(),
        }
    }

    pub fn clone_with_custom_scope(&self, scope: HashMap<String, Slisp>) -> Self {
        Self {
            global: self.global.clone(),
            scope,
        }
    }

    pub fn get_scope(&self) -> &HashMap<String, Slisp> {
        &self.scope
    }

    pub fn add_to_global(&mut self, key: String, value: Slisp) {
        self.global.insert(key, value);
    }

    pub fn add_to_scope(&mut self, key: String, value: Slisp) {
        self.scope.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Slisp> {
        self.scope.get(key).or_else(|| self.global.get(key))
    }
}

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
                Slisp::Numeric(x)
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
