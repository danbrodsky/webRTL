#[macro_use]
extern crate nom;
use nom::{
    IResult,
    bytes,
    named,
    do_parse,
    tag,
    take_until,
    take_while1,
    character::is_alphanumeric,
    character::complete::newline,
    character::complete::line_ending,
    character::complete::alpha0,
    character::complete::alphanumeric1,
    combinator::rest,
    named_args,
    preceded,
};

pub struct LUT {
}

pub struct Register {
}

#[derive(Debug, Eq, PartialEq)]
pub struct Var  {
    name: String
}

impl Var {

    pub fn new(name: String) -> Var {
        Var{name}
    }
}

pub enum Element {
    LUT,
    Register
}

pub struct Model {
    name: String,
    inputs: Vec<Var>,
    outputs: Vec<Var>,
    elements: Vec<Element>
}

named!(string, take_while1!(is_alphanumeric));

named_args!(
    kv<'a>(key: &str)<&'a str, &'a str>,
    preceded!(tag!(key), alphanumeric1)
);

named!(
    get_model_name<&str, &str>,
    call!(kv, ".model ")
);

named!(
    alphanum, take_while!(is_alphanumeric)
);

named!(
    get_inputs<&str, Vec<Var>>,
    do_parse!(
        tag!(".inputs ") >>
        inputs: separated_list0!(tag!(" "), alphanumeric1) >>
        newline >>
        (inputs.into_iter().map(|x| Var::new(x.to_string())).collect())
    )
);

#[test]
fn test_get_inputs() {
    assert_eq!(get_inputs(".inputs in0 in1 in2\n"), Ok(("", vec!(Var{name:"in0".to_string()},
                                                               Var{name:"in1".to_string()},
                                                               Var{name:"in2".to_string()}))));
}

named!(
    get_outputs<&str, &str>,
    call!(kv, ".outputs ")
);

named!(
    get_luts<&str, &str>,
    call!(kv, ".names ")
);


#[test]
fn test_kv_parse() {
    assert_eq!(get_model_name(".model counter"), Ok(("", "counter")));
}

// named!(
//     model<Model>,
//     do_parse!(
//         tag!(".model ") >>
//         name: take_until!(newline) >>
//         inputs: take_until!(newline) >>
//         outputs: take_until!(newline) >>


//         (value.trim_end())
//     )
// );


fn main() {
    println!("Hello, world!");
}
