use crate::config::{STATE, Var};
use std::num;
use std::sync;

// pub type BoxErr = Box<dyn Error>;

#[macro_use]
macro_rules! u {
    ( $mv: expr ) => {
        $mv.lock().unwrap()
    };
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TableType {
    LUT,

}

/// Error types that can be returned in this project
#[derive(Debug)]
pub enum Error {
    /// Error when creating element
    InvalidInput(TableType),

    /// previous mutex owner panicked
    Locking,

    /// a read was made for a value that was not stored in the current state
    InvalidRead,

    /// Error occurred while parsing the blif spec
    ParsingFailed(String)


}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Error {
        Error::ParsingFailed(err.to_string())
    }
}


impl<T> From<sync::PoisonError<T>> for Error {
    fn from(err: sync::PoisonError<T>) -> Error {
        Error::Locking
    }
}


pub fn get(var: &str) -> Result<u8, Error> {
    let state = STATE.lock()?;
    Ok(state.get(var).ok_or(Error::InvalidRead)?.val)
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

    // trace!("setting {} to val {:#?}", var, val);
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
    return bv
}

pub fn to_u32(v: Vec<u8>) -> u32 {
    v.iter().rev().fold(0, |acc, &b| acc << 1 | b as u32)
}
