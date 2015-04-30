//! utils

use std::error::Error; 
use std::io::prelude::*;
// use std::io::{ BufReader, BufWriter};
// use std::str::FromStr;
// use std::fmt::{self, Formatter};
use std::fmt;
use std::mem::{transmute};


fn read_uint_4(p: &[u8]) -> Result<u32, ()> {
    Ok(get4byte(p))
}

/// convert 4 byte u8 to a u32
pub fn get4byte(p: &[u8]) -> u32 {
    assert_eq!(p.len(), 4);
    (p[0] as u32) | (p[1] as u32) << 8 | (p[2] as u32) << 16 | (p[3] as u32) << 24
}


/// convert 2 byte u8 to a u32
pub fn get2byte(p: &[u8]) -> u16 {
    assert_eq!(p.len(), 2);
    (p[0] as u16) | (p[1] as u16) << 8
}

/// convert 4 byte u8 to a u32 from reader
pub fn  readU16<B: BufRead>(reader: &mut B) ->u16 {

    let mut p = [0;2];
    for i in 0..2 {
        let mut u = [0];
        reader.read(&mut u);
        p[i] = u[0];
    }

     get2byte(&p)
}

/// convert 4 byte u8 to a f32 from reader
pub fn  readFloat32<B: BufRead>(reader: &mut B) ->f32 {

    let mut p = [0;4];
    for i in 0..4 {
        let mut u = [0];
        reader.read(&mut u);
        p[i] = u[0];
    }

    let x = read_uint_4(&p).map(|i| unsafe {
            transmute::<u32, f32>(i)
        });

    let result = match x {
        Ok(v) => {
            v
        },
        _=>{
            panic!("readFloat32 error");
        }
    };

    result
}

/// An utility function to print out all  items in the vector.
pub fn print_all<T: fmt::Display>(all: Vec<T>) {
    for a in all.iter() {
        print!("{},", a);
    }
}

/// A temporary replacement of unstable vec::push_all
pub fn vec_push_all<T: Clone>(all: &mut Vec<T>, other: &[T]) {
    for i in 0..other.len() {
        all.push(other[i].clone());
    }
}
