//! utils

use std::io::prelude::*;
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

pub fn put4byte(i: u32, p: &mut [u8]) {
    p[0] = (i & 0x000000FF) as u8;
    p[1] = ((i & 0x0000FF00) >> 8) as u8 ;
    p[2] = ((i & 0x0000FF00) >> 16) as u8;
    p[3] = ((i & 0xFF000000) >> 24) as u8;
}

/// convert 2 byte u8 to a u32
pub fn get2byte(p: &[u8]) -> u16 {
    assert_eq!(p.len(), 2);
    (p[0] as u16) | (p[1] as u16) << 8
}

pub fn put2byte(i: u16, p: &mut [u8]) {
    p[0] = (i &0x00FF) as u8;
    p[1] = (i >> 8) as u8 ;
}

#[test]
fn test_get4byte() {
    // $python
    // >>> hex(86)
    // '0x56'
    // >>> hex(35)
    // '0x23'
    // >>> 0x002356
    // 9046
    let h:[u8;4] = [0x56, 0x23,0x0, 0x0];
    let n:[u8;4] = [86, 35, 0, 0];

    assert_eq!(get4byte(&h), 9046 );
    assert_eq!(get4byte(&n), 9046 );
}

#[test]
fn test_put4byte() {
    // $python
    // >>> hex(86)
    // '0x56'
    // >>> hex(35)
    // '0x23'
    // >>> 0x002356
    // 9046
    let mut h : [u8;4] = [10;4];;
    put4byte(9046, &mut h);

    assert_eq!(h[0], 86 );
    assert_eq!(h[1], 35 );
    assert_eq!(h[2], 0 );
    assert_eq!(h[3], 0 );
}

#[test]
fn test_put2byte() {
    let mut h : [u8;2] = [10;2];;
    put2byte(9046u16, &mut h);

    assert_eq!(h[0], 86 );
    assert_eq!(h[1], 35 );
}

/// convert 4 byte u8 to a u32 from reader
pub fn  read_u16<B: BufRead>(reader: &mut B) ->u16 {

    let mut p = [0;2];
    for i in 0..2 {
        let mut u = [0];
        reader.read(&mut u).unwrap();
        p[i] = u[0];
    }

     get2byte(&p)
}


/// convert 2 byte u8 to a u32 from reader
pub fn  write_u16<B: Write>(writer: &mut B, i: u16)  {
    let mut p = [0;2];
     put2byte(i, &mut p);
     writer.write(&mut p).unwrap();
}


/// convert 4 byte u8 to a f32 from reader
pub fn  read_float32<B: BufRead>(reader: &mut B) ->f32 {

    let mut p = [0;4];
    for i in 0..4 {
        let mut u = [0];
        reader.read(&mut u).unwrap();
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
            panic!("read_float32 error");
        }
    };

    result
}

pub fn  write_float32<B: Write>(writer: &mut B, i: f32)  {
    // TODO
}


pub fn  write_u32<B: Write>(writer: &mut B, i: u32)  {

    let mut p = [0;4];
     put4byte(i, &mut p);
     writer.write(&mut p).unwrap();
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
