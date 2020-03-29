use crate::data_structure::*;
use std::rc::Rc;

pub fn lambda(_context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    let lambda_arg = if let Slisp::Atom(s) = arguments
        .next()
        .expect("Wrong number of arguments in lambda definition")
    {
        s
    } else {
        panic!("Lambda var must be an atom")
    };
    let lambda_body = arguments
        .next()
        .expect("Wrong number of arguments in lambda definition");

    Slisp::Func(LispFunction(Rc::new(Box::new(
        move |context: &mut Context, args: Vec<Slisp>| {
            let mut arguments = args.into_iter();

            let arg = get_value(
                context,
                arguments
                    .next()
                    .expect("Wrong number of arguments in lambda call"),
            );

            let tmp = context.remove(&lambda_arg);
            context.insert(lambda_arg.clone(), arg);

            let mut result = get_value(context, lambda_body.clone());

            if let Slisp::Func(f) = &result {
                let args: Vec<Slisp> = arguments.collect();
                if args.len() > 0 {
                    result = f.0(context, args);
                }
            }

            context.remove(&lambda_arg);

            if let Some(tmp_var) = tmp {
                context.insert(lambda_arg.clone(), tmp_var);
            }
            result
        },
    ))))
}
