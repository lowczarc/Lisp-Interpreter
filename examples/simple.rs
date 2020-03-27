use lisp_interpreter::*;
use std::env::args;

fn main() {
    let first_arg = if let Some(arg) = args().skip(1).next() {
        arg.to_string()
    } else {
        panic!("Need an argument");
    };

    execute(first_arg);
}
