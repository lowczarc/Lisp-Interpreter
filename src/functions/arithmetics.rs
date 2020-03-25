use crate::data_structure::*;

pub fn equal(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = None;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Numeric(x) => {
                if let Some(y) = res {
                    if y != x {
                        return Slisp::Numeric(0);
                    }
                } else {
                    res = Some(x);
                }
            }
            x => {
                panic!("equal: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn greater(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = None;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Numeric(x) => {
                if let Some(y) = res {
                    if y <= x {
                        return Slisp::Numeric(0);
                    }
                }
                res = Some(x);
            }
            x => {
                panic!("greater: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn lower(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = None;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Numeric(x) => {
                if let Some(y) = res {
                    if y >= x {
                        return Slisp::Numeric(0);
                    }
                }
                res = Some(x);
            }
            x => {
                panic!("lower: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn add(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = 0;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Numeric(x) => {
                res += x;
            }
            x => {
                panic!("add: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(res)
}

pub fn mul(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = 1;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Numeric(x) => {
                res *= x;
            }
            x => {
                panic!("mul: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(res)
}

pub fn sub(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();
    let arg1 = get_value(
        context,
        arguments
            .next()
            .expect("sub: expected at least one argument"),
    );
    let mut res = if let Slisp::Numeric(x) = arg1 {
        x
    } else {
        panic!("sub: {:?} is not a value of type Numeric", arg1);
    };

    for value in arguments {
        match get_value(context, value) {
            Slisp::Numeric(x) => {
                res -= x;
            }
            x => {
                panic!("sub: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(res)
}

pub fn div(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();
    let arg1 = arguments.next().expect("div: expected two arguments");
    let arg1 = get_value(context, arg1);
    let arg2 = arguments.next().expect("div: expected two arguments");
    let arg2 = get_value(context, arg2);

    match (arg1, arg2) {
        (Slisp::Numeric(x1), Slisp::Numeric(x2)) => Slisp::Numeric(x1 / x2),
        _ => {
            panic!("div: expected two values of type Numeric");
        }
    }
}
