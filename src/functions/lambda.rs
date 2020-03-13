use crate::data_structure::*;
use std::rc::Rc;

pub fn lambda(context: &mut Context, mut args: Vec<LispFunction>) -> Slisp {
    if args.len() < 2 {
        panic!("Wrong number of arguments in lambda definition");
    }

    let lambda_var = if let Slisp::String(s) = args[0].0(context, Vec::new()) {
        s
    } else {
        panic!("Wrong Type in Lambda var")
    };
    let func_def = args.remove(1);

    Slisp::Func(LispFunction(Rc::new(Box::new(
        move |context: &mut Context, mut args: Vec<LispFunction>| {
            if args.len() < 1 {
                panic!("Wrong number of arguments in lambda call");
            }
            let mut arg = args.remove(0).0(context, Vec::new());

            if let Slisp::String(s) = &arg {
                if let Some(v) = context.get(s) {
                    arg = v.clone();
                }
            }

            let tmp = context.remove(&lambda_var);
            context.insert(lambda_var.clone(), arg);

            let mut result = func_def.0(context, Vec::new());

            if let Slisp::Func(f) = &result {
                if args.len() > 0 {
                    result = f.0(context, args);
                }
            }

            context.remove(&lambda_var);

            if let Some(tmp_var) = tmp {
                context.insert(lambda_var.clone(), tmp_var);
            }
            result
        },
    ))))
}
