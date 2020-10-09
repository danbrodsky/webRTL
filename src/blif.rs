#[macro_use]
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

use std::collections::HashMap;


#[derive(Debug, Eq, PartialEq)]
pub struct LUT {
    inputs: Vec<Var>,
    output: Var,
    mappings: HashMap<String, u8>
}

impl LUT {

    pub fn new(inputs: Vec<&str>, output: &str, mappings: Vec<&str>) -> LUT {

        let mut lut = LUT{
            inputs: inputs.into_iter().map(|x| Var::new(x.to_string())).collect(),
            output: Var::new(output.to_string()),
            mappings: HashMap::new()
        };
        for line in mappings {
            let kv: Vec<&str> = line.split_whitespace().collect();
            let k = kv[0].to_string();
            let v = isize::from_str_radix(kv[1],2).unwrap() as u8;
            lut.mappings.insert(k, v);
        }

        lut
    }
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
        alt!(tag!(".inputs ") | tag!(".inputs")) >>
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
    get_outputs<&str, Vec<Var>>,
    do_parse!(
        alt!(tag!(".outputs ") | tag!(".outputs")) >>
        outputs: separated_list0!(tag!(" "), alphanumeric1) >>
        newline >>
        (outputs.into_iter().map(|x| Var::new(x.to_string())).collect())
    )
);

#[test]
fn test_get_outputs() {
    assert_eq!(get_outputs(".outputs out0 out1 out2\n"), Ok(("", vec!(Var{name:"out0".to_string()},
                                                               Var{name:"out1".to_string()},
                                                               Var{name:"out2".to_string()}))));
}

named!(
    get_lut<&str, LUT>,
    do_parse!(
        tag!(".names ") >>
        io: separated_list0!(tag!(" "), alphanumeric1) >>
        newline >>
        lut: separated_list0!(tag!("\n"), is_a!(" 01-")) >>
        newline >>
        (LUT::new(io[0 .. io.len() -1].to_vec(), io[io.len()-1], lut))
    )
);

#[test]
fn test_get_lut() {
    let mut lut = LUT::new(vec!("out0","out1","out2"), "return0", vec!("011 1", "100 1"));
    assert_eq!(get_lut(".names out0 out1 out2 return0\n011 1\n100 1\n.names"), Ok((".names", lut)));
}


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
