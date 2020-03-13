use std::{collections::HashMap, env::args, rc::Rc, str::Chars};

mod data_structure;
mod functions;

use data_structure::*;

fn parse_func(program: &mut Chars) -> Vec<LispStruct> {
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
                    args.push(LispStruct::String(current_arg));
                    current_arg = String::new();
                }
            }
            ('(', false) => {
                if current_arg.len() != 0 {
                    args.push(LispStruct::String(current_arg));
                    current_arg = String::new();
                }
                args.push(LispStruct::Struct(Box::new(parse_func(program))));
            }
            (')', false) => {
                if current_arg.len() != 0 {
                    args.push(LispStruct::String(current_arg));
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

fn eval(context: &mut Context, program: Vec<LispStruct>) -> Slisp {
    let mut args = program.into_iter();
    let function_name = args.next();

    let arguments: Vec<LispFunction> = args
        .map(|elem| match elem {
            LispStruct::Struct(subelem) => LispFunction(Rc::new(Box::new(
                move |context: &mut Context, _args: Vec<LispFunction>| {
                    eval(context, subelem.to_vec())
                },
            ))),
            LispStruct::String(s) => {
                if let Ok(x) = s.parse::<i32>() {
                    Slisp::Numeric(x).into()
                } else {
                    Slisp::String(s.to_string()).into()
                }
            }
        })
        .collect();

    if let Some(LispStruct::String(name)) = function_name {
        let function = if let Some(Slisp::Func(function)) = context.remove(&name.to_string()) {
            function.0
        } else {
            panic!(format!("No function named {} in the context", name));
        };


        context.insert(name, Slisp::Func(LispFunction(function.clone())));

        function(context, arguments)
    } else if let Some(LispStruct::Struct(subelem)) = function_name {
        if let Slisp::Func(f) = eval(context, subelem.to_vec()) {
            f.0(context, arguments)
        } else {
            panic!("Only names and function are callables");
        }
    } else {
        panic!("Empty function call");
    }
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
        String::from("="),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::equal)))),
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
        String::from("λ"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::lambda)))),
    );
    context.insert(
        String::from("def"),
        Slisp::Func(LispFunction(Rc::new(Box::new(functions::def)))),
    );

    let first_arg = if let Some(arg) = args().skip(1).next() {
        arg.to_string()
    } else {
        panic!("Need an argument");
    };

    let mut chars = first_arg.chars();

    let program_struct = parse_func(&mut chars);
    for func in program_struct.into_iter() {
        if let LispStruct::Struct(func) = func {
            eval(&mut context, func.to_vec());
        }
    }
}
