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
    character::complete::multispace1
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

#[derive(Debug, Eq, PartialEq)]
pub struct Register {
    input: Var,
    output: Var,
    signal: String,
    control: Var,
    init: u8 // 0 = lo, 1 = hi, 2 = don't care, 3 = unknown
}

impl Register {

    pub fn new(input: &str,
               output: &str,
               clock: Option<(&str, &str)>,
               init: Option<char>) -> Register {

        // TODO: default global clock for latch
        let mut signal = "re";
        let mut control = "NIL";
        let mut start: u8 = 3;
        match clock {
            Some(clk) => {
                signal = clk.0;
                control = clk.1;
            }
            None => {}
        };
        match init {
            Some(i) => { start = i.to_digit(10).unwrap() as u8; }
            None => {}
        };

        Register{
            input: Var::new(input),
            output: Var::new(output),
            signal: signal.to_string(),
            control: Var::new(control),
            init: start
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Var  {
    name: String
}

impl Var {

    pub fn new<S>(name: S) -> Var where S: Into<String> {
        Var{name: name.into()}
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

named!(
    get_clock<&str, (&str, &str)>,
    do_parse!(
        tag!(" ") >>
        signal: alt!(tag!("fe") | tag!("re") | tag!("ah") | tag!("al") | tag!("as")) >>
        tag!(" ") >>
        control: is_not!(" \n") >>
        ((signal, control))
    )
);


named!(
    get_reg<&str, Register>,
    do_parse!(
        tag!(".latch ") >>
        input: is_not!(" \n") >>
        tag!(" ") >>
        output: is_not!(" \n") >>
        clock: opt!(get_clock) >>
        opt!(complete!(tag!(" "))) >>
        init: opt!(one_of!("0123")) >>
        newline >>
        (Register::new(input, output, clock, init))
    )
);

#[test]
fn test_get_reg() {
    let mut reg = Register::new("$0out[8:0][8]", "out[8]", Some(("re", "clock")), Some('2'));
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8] re clock 2\n"), Ok(("", reg)));

    let mut reg = Register::new("$0out[8:0][8]", "out[8]", Some(("re", "clock")), None);
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8] re clock\n"), Ok(("", reg)));

    let mut reg = Register::new("$0out[8:0][8]", "out[8]", None, None);
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8]\n"), Ok(("", reg)));

    let mut reg = Register::new("$0out[8:0][8]", "out[8]", None, Some('2'));
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8] 2\n"), Ok(("", reg)));
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
