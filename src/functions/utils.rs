use crate::data_structure::*;

pub fn if_else(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    if args.len() < 2 {
        panic!("Wrong number of arguments in if");
    }

    match args[0].0(context, Vec::new()) {
        Slisp::Numeric(0) | Slisp::None => {
            if let Some(f_else) = args.get(2) {
                return f_else.0(context, Vec::new());
            }
        }
        _ => return args[1].0(context, Vec::new()),
    }

    Slisp::None
}

pub fn print(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    for arg in args.iter() {
        match arg.0(context, Vec::new()) {
            Slisp::Numeric(x) => {
                print!("{}", x);
            }
            Slisp::String(s) => {
                if let Some(Slisp::Numeric(x)) = context.get(&s) {
                    print!("{}", x);
                } else {
                    print!("{}", s);
                }
            }
            _ => {}
        }
    }

    println!("");

    Slisp::None
}
