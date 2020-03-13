use crate::data_structure::*;

pub fn equal(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = None;

    for i in 0..args.len() {
        let value = args[i].0(context, Vec::new());
        match get_value(&context, value) {
            Slisp::Numeric(x) => {
                if res.is_none() {
                    res = Some(x);
                } else if let Some(y) = res {
                    if y != x {
                        return Slisp::Numeric(0);
                    }
                }
            }
            x => {
                panic!("=: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(1)
}

pub fn add(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = 0;

    for i in 0..args.len() {
        let value = args[i].0(context, Vec::new());
        match get_value(&context, value) {
            Slisp::Numeric(x) => {
                res += x;
            }
            x => {
                panic!("+: {:?} is not a value of type Numeric", x);
            }
        }
    }

    Slisp::Numeric(res)
}

pub fn sub(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = 0;

    for i in 0..args.len() {
        let value = args[i].0(context, Vec::new());
        match get_value(&context, value) {
            Slisp::Numeric(x) => {
                res -= x;
            }
            x => {
                panic!("-: {:?} is not a value of type Numeric", x);
            }
        }
        if i == 0 {
            res = -res;
        }
    }

    Slisp::Numeric(res)
}
