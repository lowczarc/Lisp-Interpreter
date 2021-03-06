use std::{rc::Rc, str::Chars};

mod data_structure;
pub mod functions;

pub use data_structure::*;

pub fn parse_lisp(program: &mut Chars) -> Vec<Slisp> {
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

pub fn execute(program: String) {
    let mut context = Context::new();

    context.add_to_global(
        String::from("+"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::add)))),
    );
    context.add_to_global(
        String::from("-"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::sub)))),
    );
    context.add_to_global(
        String::from("*"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::mul)))),
    );
    context.add_to_global(
        String::from("/"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::div)))),
    );
    context.add_to_global(
        String::from("="),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::equal)))),
    );
    context.add_to_global(
        String::from(">"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::greater)))),
    );
    context.add_to_global(
        String::from("<"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::lower)))),
    );
    context.add_to_global(
        String::from("+."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::fadd)))),
    );
    context.add_to_global(
        String::from("-."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::fsub)))),
    );
    context.add_to_global(
        String::from("*."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::fmul)))),
    );
    context.add_to_global(
        String::from("/."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::fdiv)))),
    );
    context.add_to_global(
        String::from("=."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::fequal)))),
    );
    context.add_to_global(
        String::from(">."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::fgreater)))),
    );
    context.add_to_global(
        String::from("<."),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::flower)))),
    );
    context.add_to_global(
        String::from("if"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::if_else)))),
    );
    context.add_to_global(
        String::from("print"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::print)))),
    );
    context.add_to_global(
        String::from("list"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::list)))),
    );
    context.add_to_global(
        String::from("len"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::len)))),
    );
    context.add_to_global(
        String::from("last"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::last)))),
    );
    context.add_to_global(
        String::from("push"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::push)))),
    );
    context.add_to_global(
        String::from("pop"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::pop)))),
    );
    let lambda = Slisp::Func(LispFunction(Rc::new(Box::new(functions::lambda))));
    context.add_to_global(String::from("lambda"), lambda.clone());
    context.add_to_global(String::from("λ"), lambda);
    context.add_to_global(
        String::from("def"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::def)))),
    );

    context.add_to_global(String::from("nil"), Slisp::None);

    let mut chars = program.chars();

    let program_struct = parse_lisp(&mut chars);

    for func in program_struct.into_iter() {
        if let Slisp::List(func) = func {
            functions::eval(&mut context, func.to_vec());
        }
    }
}
