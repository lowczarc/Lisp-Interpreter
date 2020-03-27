use crate::data_structure::*;

pub fn if_else(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let mut arguments = args.into_iter();

    let cond = arguments.next().expect("Wrong number of arguments in if");
    let value_if = arguments.next().expect("Wrong number of arguments in if");
    let value_else = arguments.next();

    match get_value(context, cond) {
        Slisp::Numeric(0) | Slisp::None => {
            if let Some(value) = value_else {
                get_value(context, value)
            } else {
                Slisp::None
            }
        }
        _ => get_value(context, value_if),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    let ret = args
        .into_iter()
        .map(|arg| format!("{}", get_value(context, arg)))
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", ret);

    Slisp::None
}

#[cfg(target_arch = "wasm32")]
pub fn print(context: &mut Context, args: Vec<Slisp>) -> Slisp {
    use crate::wasm::console_log;

    let ret = args
        .into_iter()
        .map(|arg| format!("{}", get_value(context, arg)))
        .collect::<Vec<String>>()
        .join(" ");

    console_log(&ret);

    Slisp::None
}
