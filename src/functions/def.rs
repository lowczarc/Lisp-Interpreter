use crate::data_structure::*;

pub fn def(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    let name_var =
        if let Slisp::Atom(s) = arguments.next().expect("Wrong number of argument in def") {
            s
        } else {
            panic!("Wrong Type in Name var")
        };

    let var = get_value(
        context,
        arguments.next().expect("Wrong number of arguments in def"),
    );

    context.add_to_global(name_var, var);

    Slisp::None
}
