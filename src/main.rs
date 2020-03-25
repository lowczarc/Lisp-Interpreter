use std::{collections::HashMap, env::args, rc::Rc, str::Chars};

mod data_structure;
mod functions;

use data_structure::*;

fn parse_lisp(program: &mut Chars) -> Vec<Slisp> {
    let mut current_arg = String::new();
    let mut args = Vec::new();
    let mut quote = false;

    while let Some(c) = program.next() {
        match (c, quote) {
            ('"', _) => {
                quote = !quote;
            }
            (c, true) => {
                current_arg.push(c);
            }
            (' ', false) | ('\n', false) | ('\t', false) => {
                if current_arg.len() != 0 {
                    args.push(Slisp::Atom(current_arg));
                    current_arg = String::new();
                }
            }
            ('(', false) => {
                if current_arg.len() != 0 {
                    args.push(Slisp::Atom(current_arg));
                    current_arg = String::new();
                }
                args.push(Slisp::List(parse_lisp(program)));
            }
            (')', false) => {
                if current_arg.len() != 0 {
                    args.push(Slisp::Atom(current_arg));
                }

                break;
            }
            (c, false) => {
                current_arg.push(c);
            }
        }
    }

    return args;
}

fn main() {
    let mut context = HashMap::new();

    context.insert(
        String::from("+"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::add)))),
    );
    context.insert(
        String::from("-"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::sub)))),
    );
    context.insert(
        String::from("*"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::mul)))),
    );
    context.insert(
        String::from("/"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::div)))),
    );
    context.insert(
        String::from("="),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::equal)))),
    );
    context.insert(
        String::from(">"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::greater)))),
    );
    context.insert(
        String::from("<"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::lower)))),
    );
    context.insert(
        String::from("if"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::if_else)))),
    );
    context.insert(
        String::from("print"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::print)))),
    );
    context.insert(
        String::from("list"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::list)))),
    );
    context.insert(
        String::from("len"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::len)))),
    );
    context.insert(
        String::from("last"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::last)))),
    );
    context.insert(
        String::from("push"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::push)))),
    );
    context.insert(
        String::from("pop"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::pop)))),
    );
    context.insert(
        String::from("λ"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::lambda)))),
    );
    context.insert(
        String::from("def"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::def)))),
    );

    context.insert(String::from("nil"), Slisp::None);

    let first_arg = if let Some(arg) = args().skip(1).next() {
        arg.to_string()
    } else {
        panic!("Need an argument");
    };

    let mut chars = first_arg.chars();

    let program_struct = parse_lisp(&mut chars);
    for func in program_struct.into_iter() {
        if let Slisp::List(func) = func {
            functions::eval(&mut context, func.to_vec());
        }
    }
}
