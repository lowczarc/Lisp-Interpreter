use crate::data_structure::*;

pub fn equal(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = None;

    for i in 0..args.len() {
        match args[i].0(context, Vec::new()) {
            Slisp::Numeric(x) => {
                if res.is_none() {
                    res = Some(x);
                } else if let Some(y) = res {
                    if y != x {
                        return Slisp::Numeric(0);
                    }
                }
            }
            Slisp::String(s) => {
                if let Some(Slisp::Numeric(x)) = context.get(&s) {
                    if res.is_none() {
                        res = Some(*x);
                    } else if let Some(y) = res {
                        if y != *x {
                            return Slisp::Numeric(0);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Slisp::Numeric(1)
}

pub fn add(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = 0;

    for i in 0..args.len() {
        match args[i].0(context, Vec::new()) {
            Slisp::Numeric(x) => {
                res += x;
            }
            Slisp::String(s) => {
                if let Some(Slisp::Numeric(x)) = context.get(&s) {
                    res += x;
                }
            }
            _ => {}
        }
    }

    Slisp::Numeric(res)
}

pub fn sub(context: &mut Context, args: Vec<LispFunction>) -> Slisp {
    let mut res = 0;

    for i in 0..args.len() {
        match args[i].0(context, Vec::new()) {
            Slisp::Numeric(x) => {
                res -= x;
            }
            Slisp::String(s) => {
                if let Some(Slisp::Numeric(x)) = context.get(&s) {
                    res -= x;
                }
            }
            _ => {}
        }
        if i == 0 {
            res = -res;
        }
    }

    Slisp::Numeric(res)
}
