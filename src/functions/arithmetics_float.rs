use crate::data_structure::*;

pub fn fequal(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = None;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Float(x) => {
                if let Some(y) = res {
                    if y != x {
                        return Slisp::Numeric(0);
                    }
                } else {
                    res = Some(x);
                }
            }
            x => {
                panic!("equal: {:?} is not a value of type Float", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn fgreater(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = None;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Float(x) => {
                if let Some(y) = res {
                    if y <= x {
                        return Slisp::Numeric(0);
                    }
                }
                res = Some(x);
            }
            x => {
                panic!("greater: {:?} is not a value of type Float", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn flower(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = None;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Float(x) => {
                if let Some(y) = res {
                    if y >= x {
                        return Slisp::Numeric(0);
                    }
                }
                res = Some(x);
            }
            x => {
                panic!("lower: {:?} is not a value of type Float", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn fadd(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = 0.;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Float(x) => {
                res += x;
            }
            x => {
                panic!("add: {:?} is not a value of type Float", x);
            }
        }
    }

    Slisp::Float(res)
}

pub fn fmul(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut res = 1.;

    for value in args.into_iter() {
        match get_value(context, value) {
            Slisp::Float(x) => {
                res *= x;
            }
            x => {
                panic!("mul: {:?} is not a value of type Float", x);
            }
        }
    }

    Slisp::Float(res)
}

pub fn fsub(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();
    let arg1 = get_value(
        context,
        arguments
            .next()
            .expect("sub: expected at least one argument"),
    );
    let mut res = if let Slisp::Float(x) = arg1 {
        x
    } else {
        panic!("sub: {:?} is not a value of type Float", arg1);
    };

    for value in arguments {
        match get_value(context, value) {
            Slisp::Float(x) => {
                res -= x;
            }
            x => {
                panic!("sub: {:?} is not a value of type Float", x);
            }
        }
    }

    Slisp::Float(res)
}

pub fn fdiv(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();
    let arg1 = arguments.next().expect("div: expected two arguments");
    let arg1 = get_value(context, arg1);
    let arg2 = arguments.next().expect("div: expected two arguments");
    let arg2 = get_value(context, arg2);

    match (arg1, arg2) {
        (Slisp::Float(x1), Slisp::Float(x2)) => Slisp::Float(x1 / x2),
        _ => {
            panic!("div: expected two values of type Float");
        }
    }
}
