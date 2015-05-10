//! utils

use std::fmt;

mod buffer;
pub use self::buffer::{
    get_4bytes,
    put_4bytes,
    read_u16,
    write_u16,
    read_float32,
    write_float32,
    write_u32,
};


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
