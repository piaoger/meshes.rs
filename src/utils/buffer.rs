

use std::io::prelude::*;
use std::mem::{transmute};


// read from 4 bytes 
fn read_uint_4(p: &[u8]) -> Result<u32, ()> {
    Ok(get_4bytes(p))
}

/// convert 4 byte u8 to a u32
pub fn get_4bytes(p: &[u8]) -> u32 {
    assert_eq!(p.len(), 4);
    (p[0] as u32) | (p[1] as u32) << 8 | (p[2] as u32) << 16 | (p[3] as u32) << 24
}

pub fn put_4bytes(i: u32, p: &mut [u8]) {
    p[0] = (i & 0x000000FF) as u8;
    p[1] = ((i & 0x0000FF00) >> 8) as u8 ;
    p[2] = ((i & 0x00FF0000) >> 16) as u8;
    p[3] = ((i & 0xFF000000) >> 24) as u8;
}

/// convert 2 byte u8 to a u32
pub fn get_2bytes(p: &[u8]) -> u16 {
    assert_eq!(p.len(), 2);
    (p[0] as u16) | (p[1] as u16) << 8
}

pub fn put_2bytes(i: u16, p: &mut [u8]) {
    p[0] = (i &0x00FF) as u8;
    p[1] = (i >> 8) as u8 ;
}


pub fn get_float32(p: &[u8]) ->f32 {
    let x = read_uint_4(&p).map(|i| unsafe {
            transmute::<u32, f32>(i)
        }).unwrap();

    x
}


pub fn put_float32(i: f32, p: &mut [u8]) {
    unsafe {
        put_4bytes(transmute(i), p);
    }
}


#[test]
fn test_get_4bytes() {
    // $python
    // >>> hex(86)
    // '0x56'
    // >>> hex(35)
    // '0x23'
    // >>> 0x002356
    // 9046
    let h:[u8;4] = [0x56, 0x23,0x0, 0x0];
    let n:[u8;4] = [86, 35, 0, 0];

    assert_eq!(get_4bytes(&h), 9046 );
    assert_eq!(get_4bytes(&n), 9046 );
}

#[test]
fn test_put_4bytes() {
    // $python
    // >>> hex(86)
    // '0x56'
    // >>> hex(35)
    // '0x23'
    // >>> 0x002356
    // 9046
    let mut h : [u8;4] = [10;4];
    put_4bytes(9046, &mut h);

    assert_eq!(h[0], 86 );
    assert_eq!(h[1], 35 );
    assert_eq!(h[2], 0 );
    assert_eq!(h[3], 0 );
}

#[test]
fn test_put_2bytes() {
    let mut h : [u8;2] = [10;2];
    put_2bytes(9046u16, &mut h);

    assert_eq!(h[0], 86 );
    assert_eq!(h[1], 35 );
}

#[test]
fn test_put_f32() {
    // 0x4000d5c9
    // 2.1224234

    // 0x3f7d31f4 
    // 0.9890435
    let mut h : [u8;4] = [0x0;4];
    put_float32(0.9890435, &mut h);

    assert_eq!(h[0], 0xf4 );
    assert_eq!(h[1], 0x31 );
    assert_eq!(h[2], 0x7d );
    assert_eq!(h[3], 0x3f );
}

/// convert 4 byte u8 to a u32 from reader
pub fn read_u16<B: BufRead>(reader: &mut B) ->u16 {

    let mut p = [0;2];
    for i in 0..2 {
        let mut u = [0];
        reader.read(&mut u).unwrap();
        p[i] = u[0];
    }

     get_2bytes(&p)
}

/// convert 2 byte u8 to a u32 from reader
pub fn  write_u16<B: Write>(writer: &mut B, i: u16)  {
    let mut p = [0;2];
     put_2bytes(i, &mut p);
     writer.write(&mut p).unwrap();
}


/// convert 4 byte u8 to a f32 from reader
pub fn read_float32<B: BufRead>(reader: &mut B) ->f32 {

    let mut p = [0;4];
    for i in 0..4 {
        let mut u = [0];
        reader.read(&mut u).unwrap();
        p[i] = u[0];
    }

    get_float32(&p)
}

pub fn write_float32<B: Write>(writer: &mut B, i: f32)  {
    let mut p = [0;4];
    put_float32(i, &mut p);
    writer.write(&mut p).unwrap();
}

pub fn  write_u32<B: Write>(writer: &mut B, i: u32)  {
    let mut p = [0;4];
     put_4bytes(i, &mut p);
     writer.write(&mut p).unwrap();
}