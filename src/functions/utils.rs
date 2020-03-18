use crate::data_structure::*;

pub fn if_else(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    if args.len() < 2 {
        panic!("Wrong number of arguments in if");
    }

    let val = args[0].0(context, Vec::new());
    let ret_value = match get_value(&context, val) {
        Slisp::Numeric(0) | Slisp::None => {
            if let Some(f_else) = args.get(2) {
                f_else.0(context, Vec::new())
            } else {
                Slisp::None
            }
        }
        _ => args[1].0(context, Vec::new()),
    };

    get_value(&context, ret_value)
}

pub fn print(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let ret = args.iter().map(|arg| {
        let value = arg.0(context, Vec::new());
        format!("{}", get_value(&context, value))
    }).collect::<Vec<String>>().join(" ");

    println!("{}", ret);

    Slisp::None
}
