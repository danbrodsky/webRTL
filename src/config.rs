use nom::{
    character::complete::newline,
    combinator::opt,
    combinator::complete,
    multi::many1,
    branch::alt
};

use std::collections::HashMap;
use std::sync::Mutex;
use std::mem;
use core::fmt::Debug;
use crate::util::*;

lazy_static! {
    /// Stores the current state of all signals
    pub static ref STATE: Mutex<HashMap<String, Var>> =
        Mutex::new(HashMap::new());
}

/////////////////////////////////////////////////////////////////////////////

/// Lookup (Truth) table for mapping input signals to output signals
/// only stores data on mapping, signal values are stored in STATE
#[derive(Debug, Eq, PartialEq, Default)]
pub struct LUT {
    inputs: Vec<Var>,
    output: Var,
    mappings: HashMap<Vec<u8>, u8>
}


impl LUT {

    pub fn new(inputs: Vec<&str>, output: &str, mappings: Vec<&str>)
               -> Result<LUT, Error> {

        const E: Error = Error::InvalidInput(TableType::LUT);

        let mut lut = LUT{
            inputs: inputs
                .into_iter()
                .try_fold::<_,_,Result<_,Error>>(vec![],|mut acc, x| {
                    acc.push(Var::new(x.to_string())?);
                    Ok(acc)
                })?,
            output: Var::new(output.to_string())?,
            mappings: HashMap::new()
        };
        for line in mappings {
            let kv: Vec<&str> = line.split_whitespace().collect();
            let k = kv[0]
                .chars()
                .try_fold::<_,_,Result<_,Error>>(
                    vec![], |mut acc: Vec<u8>, c: char| {
                        acc.push(c.to_digit(2).ok_or(E)? as u8);
                        Ok(acc)
                    })?;

            let v = isize::from_str_radix(kv[1],2).map_err(|_| E)? as u8;
            lut.mappings.insert(k, v);
        }

        Ok(lut)
    }

    /// executes the LUT, setting the output signal based on current input
    fn exec(&self) -> Result<(), Error> {

        let mut signals: Vec<u8> = vec!();
        for var in &self.inputs {
            let model_var = get(&var.name);
            match model_var {
                Ok(val) => signals.push(val),
                Err(e) => return Err(e)
            };
        }

        match self.mappings.get(&signals) {
            Some(&v) => {
                set(&self.output.name, v);
                trace!("{} set to high", self.output.name);
            },
            None => {
                set(&self.output.name, 0);
            }
        };

        Ok(())
    }
}


/// Direct mapping of input signal to output signal based on clock/hardware
/// Rarely used in design, typically maps signals to start values at cycle end
#[derive(Debug, Eq, PartialEq)]
pub struct Register {
    input: Vec<Var>,
    output: Var,
    signal: String,
    control: Var,
    init: u8 // 0 = lo, 1 = hi, 2 = don't care, 3 = unknown
}

impl Register {

    pub fn new(input: &str,
               output: &str,
               clock: Option<(&str, &str)>,
               init: Option<char>) -> Result<Register, Error> {

        const E: Error = Error::InvalidInput(TableType::LUT);

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
            Some(i) => { start = i.to_digit(10).ok_or(E)? as u8; }
            None => {}
        };

        Ok(Register{
            input: vec!(Var::new(input)?),
            output: Var::new(output)?,
            signal: signal.to_string(),
            control: Var::new(control)?,
            init: start
        })
    }

    fn exec(&self) -> Result<(), Error> {
        // TODO: handle varying clock triggers if possible
        trace!("{} set to {}", self.output.name, get(&self.input[0].name)?);
        let val = get(&self.input[0].name)?;
        // mem::drop(state);
        set(&self.output.name, val);

        Ok(())
    }
}

/// Basic signal in design, holds only metadata while value is in STATE
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Var  {
    pub name: String,
    pub val: u8,
    pub src: usize
}

impl Var {

    pub fn new<S>(name: S) -> Result<Var, Error> where S: Into<String> {
        let n: String = name.into();
        let mut state = STATE.lock()?;
        if let Some(v) = state.get(&n) {
            // TODO: structs should just store name of var instead of copy
            Ok(Var{name: v.name.clone(),
                   val: v.val.clone(),
                   src: v.src.clone()})
        }
        else {
            let v = Var{name: n.clone(), val: 0, src: 0};
            state.insert(n.clone(), v);
            Ok(Var{name: n, val: 0, src: 0})
        }

    }

}

/// enumeration of possible Basic Logic Elements (BLE)
#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    LUT(LUT),
    Register(Register)
}

impl Element {
    fn exec(&self) -> Result<(), Error> {
        match self {
            Element::LUT(l) => l.exec(),
            Element::Register(r) => r.exec()
        }
    }
}

/// Holds complete model representation
#[derive(Debug)]
pub struct Model {
    name: String,
    inputs: Vec<Var>,
    outputs: Vec<Var>,
    elements: Vec<Element>
}

impl Model {
    pub fn new<S>(name: S,
                  inputs: Vec<Var>,
                  outputs: Vec<Var>,
                  elements: Vec<Element>) -> Self where S: Into<String> {
        Model{
            name: name.into(),
            inputs,
            outputs,
            elements
        }
    }

    // TODO: Implement place and route for FPGA memory
    // https://www.eng.uwo.ca/people/wwang/ece616a/616_extra/notes_web/5_dphysicaldesign.pdf
    /// Crappy Topological sort implementation because too lazy to match a real FPGA
    /// (For now)
    ///
    /// Matches the dependency ordering of an FPGA's BLE, but does not contain
    /// any logic related to level ordering for finding individually parallelizable
    /// groups of BLEs.
    pub fn order(mut self) -> Self {

        let mut sorted_element_idx: Vec<usize> = vec!();

        let mut seen_vars: Vec<String> =
            self.inputs.iter().map(|i| i.name.clone()).collect();

        let mut element_inputs: &Vec<Var>;
        let mut element_output: &Var;

        let mut input_counter: Vec<usize> = vec![0; self.elements.len()];

        let mut elements = self.elements;

        // get output vars from latches too since they have an initial value
        for e in &elements {
            if let Element::Register(r) = e {
                seen_vars.push(r.output.name.clone());
            }
        }

        // starting w/ input port vars, track the # of seen vars that each
        // element has a dependency on
        while !seen_vars.is_empty() {

            let v = seen_vars.remove(0);
            info!("{:#?}", seen_vars);

            for i in 0..elements.len() {
                match &elements[i] {
                    Element::LUT(l) => {
                        element_inputs = &l.inputs;
                        element_output = &l.output;
                    }
                    Element::Register(r) => {
                        element_inputs = &r.input;
                        element_output = &r.output;
                    }
                };

                // when element dependencies all seen, record element idx
                // and add output var to seen
                for e in element_inputs {
                    if v == e.name {
                        input_counter[i] += 1;
                        if input_counter[i] == element_inputs.len() {
                            sorted_element_idx.push(i);
                            seen_vars.push(element_output.name.clone());
                        }
                    }
                }
            }
        }

        let mut sorted_elements: Vec<Element> = vec!();
        for idx in sorted_element_idx {
            let dummy: LUT = Default::default();
            sorted_elements.push(
                mem::replace(&mut elements[idx], Element::LUT(dummy))
            );
        }

        self.elements = sorted_elements;

        self

    }

    // TODO: this method is for generating groupings by dependency of the BLEs
    // The idea here would be to repeat toposort but create levels i=0..N, where
    // each level i has no BLEs dependent on levels > i. Tossing these into a
    // compute shader will yield a partial parallelization similar to that of
    // a real FPGA.
    pub fn _group() {
        panic!("Not implemented!");
    }


    pub fn eval(&self) -> Result<(), Error> {
        for e in &self.elements {
            e.exec();
        }

        // TODO: check if this gets compiled in when debug is disabled
        for out in &self.outputs {
            trace!("output '{o}' value: {:#?}",
                   get(&out.name)?,
                   o=out.name);
        }
        // clear all input signals
        for inp in &self.inputs {
            set(&inp.name, 0);
        }

        Ok(())
    }
}

/// Entry for getting FPGA configuration
#[derive(Debug)]
pub struct Config {
    pub models: Vec<Model>
}


impl Config {
    pub fn new(blif: &str) -> Result<Self, Error> {
        Ok(Config{models: Config::parse_blif(blif)?})
    }

    /// parses blif-formatted data into comprising models
    pub fn parse_blif(mut input: &str) -> Result<Vec<Model>, Error> {

        let mut models: Vec<Model> = vec!();
        let mut res;

        while input.len() > 0 {
            res = opt(get_model)(input)?;
            input = res.0;
            match res.1 {
                Some(m) => models.push(m),
                None => {
                    let g = garbage_line(input).unwrap();
                    trace!("line could not be parsed, skipping: {}", g.1);
                    input = g.0;
                }
            };
        }
        trace!("Parsed configuration: {:#?}", models);
        Ok(models)
    }
}

named!(
    get_model_name<&str, &str>,
    do_parse!(
        name: preceded!(tag!(".model "), is_not!(" \n")) >>
        newline >>
        (name)
    )
);

#[test]
fn test_get_model_name() {
    assert_eq!(get_model_name(".model counter\n"), Ok(("", "counter")));
}

named!(
    get_inputs<&str, Vec<Var>>,
    do_parse!(
        alt!(tag!(".inputs ") | tag!(".inputs")) >>
        inputs: separated_list0!(tag!(" "), is_not!(" \n")) >>
        newline >>
        (inputs.into_iter()
         .fold(std::vec![],|mut acc, x| {
             acc.push(Var::new(x.to_string())
                      .expect("Parsing inputs failed"));
             acc
         })))
);

#[test]
fn test_get_inputs() {
    assert_eq!(
        get_inputs(".inputs in0 in1 in2\n"),
        Ok(("",vec!(
            Var{name:"in0".to_string(),
                src: 0,
                val: 0
            },
            Var{name:"in1".to_string(),
                src: 0,
                val: 0
            },
            Var{name:"in2".to_string(),
                src: 0,
                val: 0
            }))));
}

named!(
    get_outputs<&str, Vec<Var>>,
    do_parse!(
        alt!(tag!(".outputs ") | tag!(".outputs")) >>
        outputs: separated_list0!(tag!(" "), is_not!(" \n")) >>
        newline >>
        (outputs.into_iter()
         .fold(std::vec![],|mut acc, x| {
             acc.push(Var::new(x.to_string())
                      .expect("Parsing outputs failed"));
             acc
         })))
);

#[test]
fn test_get_outputs() {
    assert_eq!(
        get_outputs(".outputs out0 out1 out2\n"),
        Ok(("", vec!(
            Var{name:"out0".to_string(),
                src: 0,
                val: 0
            },
            Var{name:"out1".to_string(),
                src: 0,
                val: 0
            },
            Var{name:"out2".to_string(),
                src: 0,
                val: 0
            }))));
}

named!(
    get_lut<&str, Element>,
    do_parse!(
        tag!(".names ") >>
        io: separated_list0!(tag!(" "), is_not!(" \n")) >>
        newline >>
        lut: separated_list0!(tag!("\n"), is_a!(" 01-")) >>
        newline >>
            (Element::LUT(
                LUT::new(io[0 .. io.len()-1].to_vec(),
                         io[io.len()-1], lut).expect("Parsing LUT failed"))))
);

#[test]
fn test_get_lut() {
    let lut = Element::LUT(
        LUT::new(vec!("out0","out1","out2"),
                 "return0",
                 vec!("011 1", "100 1")).unwrap());
    assert_eq!(
        get_lut(".names out0 out1 out2 return0\n011 1\n100 1\nf"),
        Ok(("f", lut))
    );
}

named!(
    get_clock<&str, (&str, &str)>,
    do_parse!(
        tag!(" ") >>
            signal: alt!(tag!("fe") | tag!("re") |
                         tag!("ah") | tag!("al") | tag!("as")) >>
        tag!(" ") >>
        control: is_not!(" \n") >>
        ((signal, control))
    )
);


named!(
    get_reg<&str, Element>,
    do_parse!(
        tag!(".latch ") >>
        input: is_not!(" \n") >>
        tag!(" ") >>
        output: is_not!(" \n") >>
        clock: opt!(get_clock) >>
        opt!(complete!(tag!(" "))) >>
        init: opt!(one_of!("0123")) >>
        newline >>
            (Element::Register(Register::new(input, output, clock, init)
                               .expect("Parsing Register failed.")))
    )
);

#[test]
fn test_get_reg() -> Result<(), Error> {
    let reg = Element::Register(
        Register::new("$0out[8:0][8]",
                      "out[8]",
                      Some(("re", "clock")),
                      Some('2'))?);
    assert_eq!(
        get_reg(".latch $0out[8:0][8] out[8] re clock 2\n"), Ok(("", reg))
    );

    let reg = Element::Register(
        Register::new("$0out[8:0][8]",
                      "out[8]",
                      Some(("re", "clock")),
                      None)?);
    assert_eq!(
        get_reg(".latch $0out[8:0][8] out[8] re clock\n"), Ok(("", reg))
    );

    let reg = Element::Register(
        Register::new("$0out[8:0][8]",
                      "out[8]",
                      None,
                      None)?);
    assert_eq!(
        get_reg(".latch $0out[8:0][8] out[8]\n"), Ok(("", reg))
    );

    let reg = Element::Register(
        Register::new("$0out[8:0][8]",
                      "out[8]",
                      None,
                      Some('2'))?);
    assert_eq!(
        get_reg(".latch $0out[8:0][8] out[8] 2\n"), Ok(("", reg))
    );

    Ok(())
}

named!(
    get_model<&str, Model>,
    do_parse!(
        name: get_model_name >>
        inputs: get_inputs >>
        outputs: get_outputs >>
        elements: many1!(complete!(alt!(get_lut | get_reg))) >>
        (Model::new(name, inputs, outputs, elements))
    )
);


#[test]
fn test_get_model() {
    // TODO: Create full test with example model
    assert!(get_model(
r#".model toplevel
.inputs clock plain[0] plain[1] plain[2] plain[3]
.outputs cipher[0] cipher[1] cipher[2] cipher[3]
.names state[3] state[2] state[1] state[0] done
1000 1
.names state[3] state[0] mod.state[0] $abc$8433$n994 $0\state[3:0][0]
0001 1
0011 1
0101 1
0110 1
0111 1
1100 1
1101 1
1110 1
1111 1
.latch $0\out[255:0][222] out[222] re clock 2
.latch $0\out[255:0][223] out[223] re clock 2
.latch $0\out[255:0][224] out[224] re clock 2
.names mod.state[0] $abc$8433$n2278 $abc$8433$n1807 $abc$8433$n1806 $abc$8433$n2289
0011 1
0111 1
1100 1
1101 1
1110 1
1111 1"#).is_ok());
}


// TODO: This fails when compiled to wasm, needs to be changed
named!(
    garbage_line<&str, &str>,
    do_parse!(
        garbage: take_until!("\n") >>
        newline >>
        (garbage)
    )
);



#[test]
fn test_parse_blif() -> Result<(), Error> {
    // TODO: test complete parsing more thoroughly
    let blif = Config::parse_blif(
r#"
# Generated by Yosys 0.9 (git sha1 UNKNOWN, gcc 10.1.0 -march=x86-64 -mtune=generic -O2 -fno-plt -fPIC -Os)

.model toplevel
.inputs clock plain[0] plain[1] plain[2] plain[3]
.outputs cipher[0] cipher[1] cipher[2] cipher[3]
.names state[3] state[0] mod.state[0] $abc$8433$n994 $0\state[3:0][0]
0001 1
0011 1
.latch $0\out[255:0][233] out[233] re clock 2
.names mod.state[1] mod.state[0] $abc$8433$n993
10 1

.model toplevel2
.inputs clock plain[0] plain[1] plain[2] plain[3]
.outputs cipher[0] cipher[1] cipher[2] cipher[3]
.names state[3] state[2] state[1] state[0] done
1000 1
.names state[3] state[0] mod.state[0] $abc$8433$n994 $0\state[3:0][0]
0001 1
0011 1
.latch $0\out[255:0][229] out[229] re clock 2
.latch $0\out[255:0][230] out[230] re clock 2
.end
"#)?;

    if blif.len() != 2 {
        assert!(false, "wrong number models returned.");
    }

    Ok(())
}

#[test]
fn test_get_element() {
    let mut parser = many1(complete(alt((get_lut, get_reg))));
    let out = parser(
r#".names mod.state[1] mod.state[0] $abc$8433$n993
10 1
.latch $0\out[255:0][229] out[229] re clock 2
.latch $0\out[255:0][230] out[230] re clock 2
"#
        );
    println!("{:#?}", out);

    assert!(out.unwrap().1.len() == 3);

}
