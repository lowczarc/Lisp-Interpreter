use crate::data_structure::*;

pub fn list(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    Slisp::List(
        args.into_iter()
            .map(|elem| get_value(context, elem))
            .collect(),
    )
}

pub fn len(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    match get_value(
        context,
        arguments.next().expect("Wrong number of arguments in len"),
    ) {
        Slisp::List(list) => Slisp::Numeric(list.len() as i32),
        _ => panic!("len argument must be a List"),
    }
}

pub fn last(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    match get_value(
        context,
        arguments.next().expect("Wrong number of arguments in last"),
    ) {
        Slisp::List(mut list) => {
            if let Some(value) = list.pop() {
                value.clone()
            } else {
                Slisp::None
            }
        }
        _ => panic!("last arguments must be a Numeric and a List"),
    }
}

pub fn push(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    let list = arguments.next().expect("Wrong number of arguments in push");
    let value = arguments.next().expect("Wrong number of arguments in push");
    match get_value(context, list) {
        Slisp::List(mut list) => {
            list.push(get_value(context, value));
            Slisp::List(list)
        }
        _ => panic!("push argument must be a List"),
    }
}

pub fn pop(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    let list = arguments.next().expect("Wrong number of arguments in pop");
    match get_value(context, list) {
        Slisp::List(mut list) => {
            list.pop();
            Slisp::List(list)
        }
        _ => panic!("pop argument must be a List"),
    }
}
