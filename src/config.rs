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

lazy_static! {
    /// Stores the current state of all signals
    pub static ref STATE: Mutex<HashMap<String, Var>> = Mutex::new(HashMap::new());
}



/////////////////////////////////////////////////////////////////////////////
// TODO: move to util.rs

#[macro_export]
macro_rules! get {
    ( $mv: expr ) => {
        STATE.lock().unwrap().get($mv).unwrap()
    };
}

pub fn get_n_to_m(var: &str, n: usize, m: usize) -> Vec<u8> {
    let s = STATE.lock().unwrap();
    let mut out = vec!();
    for b in n..m {
        out.push(s.get(&format!("{}[{}]", var, b)).unwrap().val)
    }
    return out
}

// TODO: check that var being set is Model Input
pub fn set(var: &str, val: u8) {
    STATE.lock().unwrap().get_mut(var.into()).unwrap().val = val;
}

pub fn set_n(var: &str, n: usize, val: u8) {
    STATE.lock().unwrap().get_mut(&format!("{}[{}]", var, n)).unwrap().val = val;
}

pub fn set_n_to_m(var: &str, n: usize, m: usize, val: Vec<u8>) {

    // info!("setting {} to val {:#?}", var, val);
    for b in n..m {
        STATE.lock().unwrap().get_mut(&format!("{}[{}]", var, b)).unwrap().val = val[b-n];
    }
}

// TODO: make this generic?
pub fn to_bit_vec(v: u64) -> Vec<u8> {
    let mut bv: Vec<u8> = vec!();
    let mut n = v;
    for _ in 0..64 {
        bv.push((n & 0x1) as u8);
        n = n >> 0x1;
    }
    bv
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

    pub fn new(inputs: Vec<&str>, output: &str, mappings: Vec<&str>) -> LUT {

        let mut lut = LUT{
            inputs: inputs.into_iter().map(|x| Var::new(x.to_string())).collect(),
            output: Var::new(output.to_string()),
            mappings: HashMap::new()
        };
        for line in mappings {
            let kv: Vec<&str> = line.split_whitespace().collect();
            let k = kv[0].to_string()
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u8).collect();
            let v = isize::from_str_radix(kv[1],2).unwrap() as u8;
            lut.mappings.insert(k, v);
        }

        lut
    }

    /// executes the LUT, setting the output signal based on current input
    fn exec(&self) {

        let mut signals: Vec<u8> = vec!();
        for var in &self.inputs {
            match STATE.lock().unwrap().get(&var.name) { // TODO: .lock().unwrap() as a macro possible?
                Some(mv) => signals.push(mv.val),
                None => panic!("var '{}' was not initialized", var.name)
            };
        }

        // TODO: replace STATE.lock().unwrap()... with util function
        match self.mappings.get(&signals) {
            Some(&v) => {
                set(&self.output.name, v);
                info!("{} set to high", self.output.name);
            },
            None => {
                set(&self.output.name, 0);
            }
        };
    }
}


/// Direct mapping of input signal to output signal based on clock or other hardware
/// Rarely used in design, usually just maps signals to start at end of cycle
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
            input: vec!(Var::new(input)),
            output: Var::new(output),
            signal: signal.to_string(),
            control: Var::new(control),
            init: start
        }
    }

    fn exec(&self) {
        // TODO: handle varying clock triggers if possible
        info!("{} set to {}", self.output.name, get!(&self.input[0].name).val);
        let state = STATE.lock().unwrap();
        let val = state.get(&self.input[0].name).unwrap().val;
        mem::drop(state);
        set(&self.output.name, val);
    }
}

/// Basic signal in design, holds only metadata while value is in STATE
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Var  {
    name: String,
    val: u8,
    src: usize
}

impl Var {

    pub fn new<S>(name: S) -> Var where S: Into<String> {
        let n: String = name.into();
        let mut state = STATE.lock().unwrap();
        if let Some(v) = state.get(&n) {
            // TODO: structs should just store name of var instead of copy
            Var{name: v.name.clone(), val: v.val.clone(), src: v.src.clone()}
        }
        else {
            let v = Var{name: n.clone(), val: 0, src: 0};
            state.insert(n.clone(), v);
            Var{name: n, val: 0, src: 0}
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
    fn exec(&self) {
        match self {
            Element::LUT(l) => l.exec(),
            Element::Register(r) => r.exec()
        };
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
    pub fn order(mut self) -> Self {

        let mut sorted_element_idx: Vec<usize> = vec!();

        let mut seen_vars: Vec<String> = self.inputs.iter().map(|i| i.name.clone()).collect();
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
            sorted_elements.push(mem::replace(&mut elements[idx], Element::LUT(dummy)));
        }

        self.elements = sorted_elements;

        self

    }

    pub fn eval(&self) {
        for e in &self.elements {
            e.exec();
        }

        // TODO: check if this get compiled in when debug is disabled
        for out in &self.outputs {
            info!("output '{o}' value: {:#?}", STATE.lock().unwrap().get(&out.name).unwrap(),
                  o=out.name);
        }
        // clear all input signals
        for inp in &self.inputs {
            set(&inp.name, 0);
        }
    }
}

/// Entry for getting FPGA configuration
pub struct Config {
    pub models: Vec<Model>
}


impl Config {
    pub fn new(blif: &str) -> Self {
        Config{models: Config::parse_blif(blif)}
    }

    /// parses blif-formatted data into comprising models
    pub fn parse_blif(mut input: &str) -> Vec<Model> {

        let mut models: Vec<Model> = vec!();
        let mut res;

        while input.len() > 0 {
            res = opt(get_model)(input).unwrap();
            input = res.0;
            match res.1 {
                Some(m) => models.push(m),
                None => {
                    let g = garbage_line(input).unwrap();
                    info!("line could not be parsed, skipping: {}", g.1);
                    input = g.0;
                }
            };
        }
        info!("Parsed configuration: {:#?}", models);
        models
    }
}


// TODO:
// - add parsing for inner models [x] (there are no inner modules since design is flattened)
// - form a graph structure representing order of true dependencies of different blocks [x] (yosys already does this)
// - implement LUT function for giving output on given input [x]
// eval loop for executing configuration [x]
// a single cycle should run all LUTs (now CLBs) and IOBs [x]
// - create IOB logic for r/w memory [x]
// output pins for drawing to screen [ ]
// The order of LUTs in blif is random and actually needs to be represented
// in a dependency structure

// Each Var needs to hold its bit value [x]
// assume single clock for now

// never use traits for object instantiation, self-referencing traits are a bitch



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
        (inputs.into_iter().map(|x| Var::new(x.to_string())).collect())
    )
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
        (outputs.into_iter().map(|x| Var::new(x.to_string())).collect())
    )
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
        (Element::LUT(LUT::new(io[0 .. io.len()-1].to_vec(), io[io.len()-1], lut)))
    )
);

#[test]
fn test_get_lut() {
    let lut = Element::LUT(LUT::new(vec!("out0","out1","out2"), "return0", vec!("011 1", "100 1")));
    assert_eq!(get_lut(".names out0 out1 out2 return0\n011 1\n100 1\nf"), Ok(("f", lut)));
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
        (Element::Register(Register::new(input, output, clock, init)))
    )
);

#[test]
fn test_get_reg() {
    let mut reg = Element::Register(Register::new("$0out[8:0][8]", "out[8]", Some(("re", "clock")), Some('2')));
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8] re clock 2\n"), Ok(("", reg)));

    let mut reg = Element::Register(Register::new("$0out[8:0][8]", "out[8]", Some(("re", "clock")), None));
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8] re clock\n"), Ok(("", reg)));

    let mut reg = Element::Register(Register::new("$0out[8:0][8]", "out[8]", None, None));
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8]\n"), Ok(("", reg)));

    let mut reg = Element::Register(Register::new("$0out[8:0][8]", "out[8]", None, Some('2')));
    assert_eq!(get_reg(".latch $0out[8:0][8] out[8] 2\n"), Ok(("", reg)));
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
fn test_parse_blif() {
    // TODO: test complete parsing more thoroughly
    let mut blif = Config::parse_blif(
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
"#);

    if blif.len() != 2 {
        assert!(false, "wrong number models returned.");
    }
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
