
//! Reader and writer for STL model

mod reader;
mod writer;

pub use self::reader::load;
pub use self::writer::save;