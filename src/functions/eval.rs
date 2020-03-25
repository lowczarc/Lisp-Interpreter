use crate::data_structure::*;

pub fn eval(context: &mut Context, program: Vec<Slisp>) -> Slisp {
    let mut args = program.into_iter();
    let function_name = args.next();
    let arguments: Vec<Slisp> = args.collect();

    if let Some(Slisp::Atom(name)) = function_name {
        let function = if let Some(Slisp::Func(function)) = context.remove(&name.to_string()) {
            function.0
        } else {
            panic!(format!("No function named {} in the context", name));
        };

        context.insert(name, Slisp::Func(LispFunction(function.clone())));

        function(context, arguments)
    } else if let Some(Slisp::List(subelem)) = function_name {
        if let Slisp::Func(f) = eval(context, subelem) {
            f.0(context, arguments)
        } else {
            panic!("Only function are callables");
        }
    } else {
        panic!("Empty function call: {:?}", function_name);
    }
}
