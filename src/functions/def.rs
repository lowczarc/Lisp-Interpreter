use crate::data_structure::*;

pub fn def(context: &mut Context, mut args: Vec<LispFunction>) -> Slisp {
    if args.len() < 2 {
        panic!("Wrong number of arguments in var definition");
    }

    let name_var = if let Slisp::Literal(s) = args[0].0(context, Vec::new()) {
        s
    } else {
        panic!("Wrong Type in Name var")
    };

    let var = args.remove(1).0(context, Vec::new());

    context.insert(name_var, var);

    Slisp::None
}
