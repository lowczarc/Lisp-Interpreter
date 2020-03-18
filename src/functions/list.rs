use crate::data_structure::*;

pub fn list(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = Vec::new();

    for arg in args.iter() {
        let value = arg.0(context, Vec::new());
        res.push(get_value(&context, value));
    }

    Slisp::List(res)
}

pub fn len(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    if args.len() == 0 {
        panic!("Wrong number of arguments in len");
    }

    let val = args[0].0(context, Vec::new());
    match get_value(&context, val) {
        Slisp::List(list) => Slisp::Numeric(list.len() as i32),
        _ => panic!("len argument must be a List"),
    }
}

pub fn last(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    if args.len() == 0 {
        panic!("Wrong number of arguments in last");
    }

    let list = args[0].0(context, Vec::new());
    match get_value(&context, list) {
        Slisp::List(mut list) => {
            if let Some(value) = list.pop() {
                value.clone()
            } else {
                Slisp::None
            }
        },
        _ => panic!("len arguments must be a Numeric and a List"),
    }
}

pub fn push(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    if args.len() < 2 {
        panic!("Wrong number of arguments in push");
    }

    let list = args[0].0(context, Vec::new());
    let value = args[1].0(context, Vec::new());
    match get_value(&context, list) {
        Slisp::List(mut list) => {
            list.push(get_value(&context, value));
            Slisp::List(list)
        },
        _ => panic!("len argument must be a List"),
    }
}

pub fn pop(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    if args.len() == 0 {
        panic!("Wrong number of arguments in pop");
    }

    let list = args[0].0(context, Vec::new());
    match get_value(&context, list) {
        Slisp::List(mut list) => {
            list.pop();
            Slisp::List(list)
        },
        _ => panic!("len argument must be a List"),
    }
}
